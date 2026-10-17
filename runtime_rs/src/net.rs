use std::net::TcpListener;
use std::io::{Read, Write, BufReader};
use std::sync::Arc;
use std::thread;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::collections::HashMap;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ServerConfig, ServerConnection, StreamOwned};

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub content_type: String,
    pub body: String,
    pub raw_bytes: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn json(status: u16, body: &str) -> Self {
        Self {
            status,
            content_type: "application/json; charset=utf-8".to_string(),
            body: body.to_string(),
            raw_bytes: None,
        }
    }

    pub fn html(status: u16, body: &str) -> Self {
        Self {
            status,
            content_type: "text/html; charset=utf-8".to_string(),
            body: body.to_string(),
            raw_bytes: None,
        }
    }

    pub fn text(status: u16, body: &str) -> Self {
        Self {
            status,
            content_type: "text/plain; charset=utf-8".to_string(),
            body: body.to_string(),
            raw_bytes: None,
        }
    }

    pub fn binary(status: u16, content_type: &str, data: Vec<u8>) -> Self {
        Self {
            status,
            content_type: content_type.to_string(),
            body: String::new(),
            raw_bytes: Some(data),
        }
    }
}

pub struct ServerThreadPool {

    sender: std::sync::mpsc::SyncSender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ServerThreadPool {
    pub fn new(workers: usize, queue_cap: usize) -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel::<Box<dyn FnOnce() + Send + 'static>>(queue_cap);
        let receiver = Arc::new(std::sync::Mutex::new(receiver));

        for _ in 0..workers {
            let rx = Arc::clone(&receiver);
            thread::spawn(move || {
                loop {
                    let job = match rx.lock() {
                        Ok(lock) => match lock.recv() {
                            Ok(j) => j,
                            Err(_) => break,
                        },
                        Err(_) => break,
                    };
                    let _ = catch_unwind(AssertUnwindSafe(job));
                }
            });
        }

        Self { sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), std::sync::mpsc::TrySendError<Box<dyn FnOnce() + Send + 'static>>>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.try_send(Box::new(f))
    }
}

pub struct HttpServer {

    pub port: u16,
    pub is_tls: bool,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        Self { 
            port,
            is_tls: false,
            tls_cert: None,
            tls_key: None,
        }
    }

    pub fn new_https(port: u16, cert: Option<String>, key: Option<String>) -> Self {
        Self {
            port,
            is_tls: true,
            tls_cert: cert,
            tls_key: key,
        }
    }

    pub fn listen<F>(&self, handler: F) 
    where F: Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static {
        let bind_addr = format!("0.0.0.0:{}", self.port);
        let listener = match TcpListener::bind(&bind_addr) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("\n[AAYU Server Error] Could not bind to port {}: {}", self.port, e);
                eprintln!("  -> Reason: {}", e);
                eprintln!("  -> Fix: Either terminate the existing process or use '--port <ANOTHER_PORT>'\n");
                std::process::exit(1);
            }
        };

        let protocol = if self.is_tls { "https" } else { "http" };
        println!("==================================================");
        println!("AAYU Native Web Server running on port {}", self.port);
        println!("Accepting connections at {}://localhost:{}", protocol, self.port);
        if self.is_tls {
            println!("Security: Native TLS 1.3 / HTTPS Encryption ACTIVE");
            if self.tls_cert.is_none() {
                println!("TLS Mode: Auto-generated ephemeral dev certificate (localhost)");
            } else {
                println!("TLS Mode: Custom PEM Certificate & Private Key");
            }
        }
        println!("==================================================");

        let tls_config: Option<Arc<ServerConfig>> = if self.is_tls {
            Some(Arc::new(self.init_tls_config()))
        } else {
            None
        };

        let handler = Arc::new(handler);
        let num_workers = (thread::available_parallelism().map(|n| n.get()).unwrap_or(8) * 8).clamp(32, 256);
        let pool = ServerThreadPool::new(num_workers, 16384);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _ = stream.set_nodelay(true);
                    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
                    let handler_clone = Arc::clone(&handler);
                    let tls_config_clone = tls_config.clone();
                    let stream_for_worker = match stream.try_clone() {
                        Ok(s) => s,
                        Err(_) => continue,
                    };

                    let sent = pool.execute(move || {
                        let mut stream = stream_for_worker;
                        let _ = catch_unwind(AssertUnwindSafe(|| {
                            if let Some(tls_cfg) = tls_config_clone {
                                if let Ok(conn) = ServerConnection::new(tls_cfg) {
                                    let mut tls_stream = StreamOwned::new(conn, stream);
                                    Self::handle_connection(&mut tls_stream, &handler_clone);
                                }
                            } else {
                                Self::handle_connection(&mut stream, &handler_clone);
                            }
                        }));
                    });

                    if sent.is_err() {
                        let _ = stream.write_all(b"HTTP/1.1 503 Service Unavailable\r\nRetry-After: 1\r\nContent-Length: 21\r\n\r\nServer Under Pressure");
                    }

                }
                Err(e) => {
                    eprintln!("[AAYU Server] TCP accept error: {}", e);
                }
            }
        }
    }


    fn init_tls_config(&self) -> ServerConfig {
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let builder = ServerConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .expect("Failed to initialize safe TLS protocol versions");

        if let (Some(cert_path), Some(key_path)) = (&self.tls_cert, &self.tls_key) {
            let cert_file = match std::fs::File::open(cert_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("\n[AAYU TLS Error] Cannot open TLS certificate file '{}': {}", cert_path, e);
                    eprintln!("  -> Tip: Omit '--cert' and '--key' to let AAYU auto-generate an in-memory dev certificate.\n");
                    std::process::exit(1);
                }
            };
            let mut cert_reader = BufReader::new(cert_file);
            let certs: Vec<CertificateDer<'static>> = match rustls_pemfile::certs(&mut cert_reader).collect::<Result<Vec<_>, _>>() {
                Ok(c) if !c.is_empty() => c,
                Ok(_) => {
                    eprintln!("\n[AAYU TLS Error] No valid certificates found in '{}'\n", cert_path);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("\n[AAYU TLS Error] Failed to parse TLS certificate '{}': {}\n", cert_path, e);
                    std::process::exit(1);
                }
            };

            let key_file = match std::fs::File::open(key_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("\n[AAYU TLS Error] Cannot open TLS private key file '{}': {}", key_path, e);
                    std::process::exit(1);
                }
            };
            let mut key_reader = BufReader::new(key_file);
            let key: PrivateKeyDer<'static> = match rustls_pemfile::private_key(&mut key_reader) {
                Ok(Some(k)) => k,
                Ok(None) => {
                    eprintln!("\n[AAYU TLS Error] No valid private key found in '{}'\n", key_path);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("\n[AAYU TLS Error] Failed to parse TLS private key in '{}': {}\n", key_path, e);
                    std::process::exit(1);
                }
            };

            builder
                .with_no_client_auth()
                .with_single_cert(certs, key)
                .unwrap_or_else(|e| {
                    eprintln!("\n[AAYU TLS Error] Failed to configure TLS certificates: {}\n", e);
                    std::process::exit(1);
                })
        } else {
            let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
            let certified_key = rcgen::generate_simple_self_signed(subject_alt_names)
                .expect("Failed to generate ephemeral dev TLS certificate");

            let cert_der = certified_key.cert.der().to_vec();
            let key_der = certified_key.signing_key.serialize_der();

            let certs = vec![CertificateDer::from(cert_der)];
            let private_key = PrivateKeyDer::Pkcs8(key_der.into());

            builder
                .with_no_client_auth()
                .with_single_cert(certs, private_key)
                .expect("Failed to configure ephemeral dev TLS certificate")
        }
    }

    fn handle_connection<S: Read + Write>(stream: &mut S, handler: &Arc<impl Fn(HttpRequest) -> HttpResponse>) {
        let mut buffer = Vec::with_capacity(8192);
        let mut temp = [0u8; 4096];
        let mut keep_alive_count = 0;
        const MAX_KEEP_ALIVE: usize = 100;

        loop {
            let mut content_length: Option<usize> = None;
            let mut header_end: Option<usize> = None;
            let mut is_keep_alive = false;
            let mut is_close = false;

            loop {
                // Check if buffer already contains a complete header
                if header_end.is_none() {
                    if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
                        header_end = Some(pos + 4);
                        let header_str = String::from_utf8_lossy(&buffer[..pos]);
                        for line in header_str.lines() {
                            let lower = line.to_ascii_lowercase();
                            if lower.starts_with("content-length:") {
                                if let Some(val_str) = line.split(':').nth(1) {
                                    if let Ok(cl) = val_str.trim().parse::<usize>() {
                                        content_length = Some(cl);
                                    }
                                }
                            } else if lower.starts_with("connection:") {
                                if lower.contains("close") {
                                    is_close = true;
                                } else if lower.contains("keep-alive") {
                                    is_keep_alive = true;
                                }
                            }
                        }
                    } else if let Some(pos) = buffer.windows(2).position(|w| w == b"\n\n") {
                        header_end = Some(pos + 2);
                    }
                }

                if let Some(hend) = header_end {
                    let current_body_len = buffer.len() - hend;
                    let required_len = content_length.unwrap_or(0);
                    if current_body_len >= required_len {
                        break;
                    }
                }

                match stream.read(&mut temp) {
                    Ok(0) => return,
                    Ok(n) => {
                        buffer.extend_from_slice(&temp[..n]);
                    }
                    Err(_) => return,
                }
            }

            let hend = match header_end {
                Some(h) => h,
                None => return,
            };
            let req_total_len = hend + content_length.unwrap_or(0);
            if req_total_len > buffer.len() { return; }

            let raw_request = String::from_utf8_lossy(&buffer[..req_total_len]).to_string();
            buffer.drain(..req_total_len);

            let req = Self::parse_request(&raw_request);

            let res = if req.method == "OPTIONS" {
                HttpResponse {
                    status: 204,
                    content_type: "text/plain".to_string(),
                    body: String::new(),
                    raw_bytes: None,
                }
            } else {
                handler(req)
            };

            let status_text = match res.status {
                200 => "OK",
                201 => "Created",
                204 => "No Content",
                400 => "Bad Request",
                401 => "Unauthorized",
                403 => "Forbidden",
                404 => "Not Found",
                500 => "Internal Server Error",
                _ => "Unknown",
            };

            let body_bytes = match &res.raw_bytes {
                Some(bytes) => bytes.as_slice(),
                None => res.body.as_bytes(),
            };

            keep_alive_count += 1;
            let should_keep_alive = !is_close && (is_keep_alive || keep_alive_count < MAX_KEEP_ALIVE);
            let connection_header = if should_keep_alive {
                "Connection: keep-alive\r\nKeep-Alive: timeout=5, max=100\r\n"
            } else {
                "Connection: close\r\n"
            };

            let response_header = format!(
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\n{}{}\r\n",
                res.status,
                status_text,
                res.content_type,
                body_bytes.len(),
                connection_header,
                ""
            );

            if stream.write_all(response_header.as_bytes()).is_err() { return; }
            if stream.write_all(body_bytes).is_err() { return; }
            if stream.flush().is_err() { return; }

            if !should_keep_alive || keep_alive_count >= MAX_KEEP_ALIVE {
                break;
            }
        }
    }

    fn parse_request(raw: &str) -> HttpRequest {
        let (headers_part, body_part) = if let Some(pos) = raw.find("\r\n\r\n") {
            (&raw[..pos], &raw[pos + 4..])
        } else if let Some(pos) = raw.find("\n\n") {
            (&raw[..pos], &raw[pos + 2..])
        } else {
            (raw, "")
        };

        let mut lines = headers_part.lines();
        let first_line = lines.next().unwrap_or("GET / HTTP/1.1");
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap_or("GET").to_string();
        let path = parts.next().unwrap_or("/").to_string();

        HttpRequest { 
            method, 
            path, 
            body: body_part.to_string() 
        }
    }
}

/// Dynamically extracts routes and their corresponding response payloads from an AAYU source file.
pub fn extract_routes_from_source(source: &str) -> HashMap<String, (String, String)> {
    let mut routes: HashMap<String, (String, String)> = HashMap::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("route ") || trimmed.starts_with("route\"") || trimmed.starts_with("route'") {
            let path_part = trimmed.trim_start_matches("route").trim();
            let path = if (path_part.starts_with('"') && path_part.ends_with('"')) ||
                          (path_part.starts_with('\'') && path_part.ends_with('\'')) {
                path_part[1..path_part.len()-1].to_string()
            } else {
                let p = path_part.split_whitespace().next().unwrap_or("/");
                p.trim_matches('"').trim_matches('\'').to_string()
            };

            let mut block_lines: Vec<String> = Vec::new();
            i += 1;
            let mut depth = 1;
            let mut in_triple = false;
            while i < lines.len() && depth > 0 {
                let l = lines[i].trim();
                let triple_count = l.matches("\"\"\"").count() + l.matches("'''").count();
                if triple_count % 2 == 1 {
                    in_triple = !in_triple;
                }
                if !in_triple {
                    if l == "end" {
                        depth -= 1;
                        if depth == 0 { break; }
                    } else if l.starts_with("route ") || l == "get" || l == "post" || l == "put" || l == "delete" || l.starts_with("action ") {
                        depth += 1;
                    }
                }
                block_lines.push(lines[i].to_string());
                i += 1;
            }

            let block_str = block_lines.join("\n");
            let mut vars: HashMap<String, String> = HashMap::new();
            let mut respond_arg = String::new();

            for line in &block_lines {
                let l = line.trim();
                if l.starts_with("let ") {
                    if let Some(eq_pos) = l.find('=') {
                        let var_name = l[4..eq_pos].trim().to_string();
                        let mut val = l[eq_pos+1..].trim().to_string();
                        if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                            val = val[1..val.len()-1].to_string();
                        }
                        vars.insert(var_name, val);
                    }
                }
            }

            if let Some(pos) = block_str.find("respond(") {
                let after = &block_str[pos + 8..];
                let trimmed = after.trim_start();
                if trimmed.starts_with("\"\"\"") {
                    let content = &trimmed[3..];
                    if let Some(end_triple) = content.rfind("\"\"\"") {
                        respond_arg = content[..end_triple].to_string();
                    } else {
                        respond_arg = content.to_string();
                    }
                } else if trimmed.starts_with("'''") {
                    let content = &trimmed[3..];
                    if let Some(end_triple) = content.rfind("'''") {
                        respond_arg = content[..end_triple].to_string();
                    } else {
                        respond_arg = content.to_string();
                    }
                } else {
                    let mut paren_depth = 1;
                    let mut end_pos = None;
                    let mut in_str = false;
                    let mut str_char = '"';
                    let mut escaped = false;

                    for (idx, ch) in after.char_indices() {
                        if escaped {
                            escaped = false;
                            continue;
                        }
                        if ch == '\\' {
                            escaped = true;
                            continue;
                        }
                        if in_str {
                            if ch == str_char {
                                in_str = false;
                            }
                            continue;
                        }
                        if ch == '"' || ch == '\'' {
                            in_str = true;
                            str_char = ch;
                            continue;
                        }
                        if ch == '(' {
                            paren_depth += 1;
                        } else if ch == ')' {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                end_pos = Some(idx);
                                break;
                            }
                        }
                    }

                    if let Some(ep) = end_pos {
                        respond_arg = after[..ep].trim().to_string();
                    }
                }
            } else if let Some(pos) = block_str.find("respond ") {
                let after = &block_str[pos + 8..];
                let trimmed = after.trim_start();
                if trimmed.starts_with("\"\"\"") {
                    let content = &trimmed[3..];
                    if let Some(end_triple) = content.rfind("\"\"\"") {
                        respond_arg = content[..end_triple].to_string();
                    } else {
                        respond_arg = content.to_string();
                    }
                } else if trimmed.starts_with("'''") {
                    let content = &trimmed[3..];
                    if let Some(end_triple) = content.rfind("'''") {
                        respond_arg = content[..end_triple].to_string();
                    } else {
                        respond_arg = content.to_string();
                    }
                } else {
                    respond_arg = after.lines().next().unwrap_or("").trim().to_string();
                }
            }

            let mut final_body = if let Some(resolved) = vars.get(&respond_arg) {
                resolved.clone()
            } else if !respond_arg.is_empty() {
                respond_arg.clone()
            } else {
                r#"{"status":"OK","engine":"AAYU Silicon"}"#.to_string()
            };

            while (final_body.starts_with('"') && final_body.ends_with('"')) ||
                  (final_body.starts_with('\'') && final_body.ends_with('\'')) {
                if final_body.len() < 2 { break; }
                final_body = final_body[1..final_body.len()-1].to_string();
            }

            // Fix Python-dict style syntax if dictionary was used in respond({ ... })
            if final_body.starts_with('{') && final_body.ends_with('}') {
                // Ensure double quotes for JSON compliance
                final_body = final_body.replace('\'', "\"");
            }

            // Content-Type detection
            let content_type = if final_body.trim().starts_with('<') || final_body.contains("<html>") || final_body.contains("<!DOCTYPE") || final_body.contains("</div>") {
                "text/html; charset=utf-8".to_string()
            } else if final_body.trim().starts_with('{') || final_body.trim().starts_with('[') {
                "application/json; charset=utf-8".to_string()
            } else {
                "text/plain; charset=utf-8".to_string()
            };

            routes.insert(path, (content_type, final_body));
        }
        i += 1;
    }

    routes
}

/// Native Outbound HTTPS / TLS Client
pub fn fetch_https(url: &str) -> Result<String, String> {
    ureq::get(url)
        .call()
        .map_err(|e| format!("HTTPS request failed: {}", e))?
        .into_string()
        .map_err(|e| format!("Failed to read HTTPS body: {}", e))
}
