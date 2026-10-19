use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};

pub mod lexer;
pub mod parser;
pub mod emitter;

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let ast = parse_source(source)?;
    if ast.iter().any(|s| matches!(s, parser::Statement::Import { .. })) {
        return Err("imports require compiling a file so relative paths can be resolved".into());
    }
    compile_ast(&ast)
}

pub fn compile_file(path: &Path) -> Result<Vec<u8>, String> {
    let ast = expand_file(path, None, &mut HashSet::new(), &mut HashSet::new())?;
    compile_ast(&ast)
}

fn parse_source(source: &str) -> Result<Vec<parser::Statement>, String> {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();
    
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    if !parser.errors().is_empty() {
        return Err(parser.errors().join("\n"));
    }
    Ok(ast)
}

fn compile_ast(ast: &[parser::Statement]) -> Result<Vec<u8>, String> {
    validate_supported(ast)?;
    let mut emitter = emitter::Emitter::new();
    let (bytecode, constants, strings, num_constants) = emitter.emit(ast);
    if bytecode.len() > u16::MAX as usize {
        return Err("compiled code exceeds the current 65535-byte address limit".into());
    }
    if num_constants > u16::MAX as u32 + 1 {
        return Err("compiled program exceeds the 65536-entry constant limit".into());
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"AAYU"); // 4 bytes header
    
    let num_strings = strings.len() as u32;
    out.extend_from_slice(&num_strings.to_le_bytes());
    for s in strings {
        let bytes = s.as_bytes();
        let len = bytes.len() as u32;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(bytes);
    }
    
    out.extend_from_slice(&num_constants.to_le_bytes());
    out.extend_from_slice(&constants);
    
    out.extend_from_slice(&bytecode);
    Ok(out)
}

fn qualify_expr(
    expr: parser::Expr,
    ns: &str,
    actions: &HashSet<String>,
    vars: &HashSet<String>,
    local_scope: &HashSet<String>,
) -> parser::Expr {
    use parser::Expr;
    match expr {
        Expr::Call { mut callee, args } => {
            let new_args = args.into_iter().map(|a| qualify_expr(a, ns, actions, vars, local_scope)).collect();
            if actions.contains(&callee) {
                callee = format!("{}.{}", ns, callee);
            }
            Expr::Call { callee, args: new_args }
        }
        Expr::Variable(name) => {
            if !local_scope.contains(&name) && vars.contains(&name) {
                Expr::Variable(format!("{}.{}", ns, name))
            } else {
                Expr::Variable(name)
            }
        }
        Expr::Unary { op, expr } => Expr::Unary { op, expr: Box::new(qualify_expr(*expr, ns, actions, vars, local_scope)) },
        Expr::Binary { left, op, right } => Expr::Binary {
            left: Box::new(qualify_expr(*left, ns, actions, vars, local_scope)),
            op,
            right: Box::new(qualify_expr(*right, ns, actions, vars, local_scope)),
        },
        Expr::Array(items) => Expr::Array(items.into_iter().map(|i| qualify_expr(i, ns, actions, vars, local_scope)).collect()),
        Expr::Dict(entries) => Expr::Dict(entries.into_iter().map(|(k, v)| (qualify_expr(k, ns, actions, vars, local_scope), qualify_expr(v, ns, actions, vars, local_scope))).collect()),
        Expr::Index { object, key } => Expr::Index {
            object: Box::new(qualify_expr(*object, ns, actions, vars, local_scope)),
            key: Box::new(qualify_expr(*key, ns, actions, vars, local_scope)),
        },
        other => other,
    }
}

fn qualify_stmt(
    stmt: parser::Statement,
    ns: &str,
    actions: &HashSet<String>,
    vars: &HashSet<String>,
    local_scope: &mut HashSet<String>,
) -> parser::Statement {
    use parser::Statement;
    match stmt {
        Statement::Action { name, args, body } => {
            let new_name = format!("{}.{}", ns, name);
            let mut inner_scope: HashSet<String> = args.iter().cloned().collect();
            let new_body = body.into_iter().map(|s| qualify_stmt(s, ns, actions, vars, &mut inner_scope)).collect();
            Statement::Action { name: new_name, args, body: new_body }
        }
        Statement::Let { name, value } => {
            if local_scope.is_empty() {
                let new_name = format!("{}.{}", ns, name);
                let new_val = qualify_expr(value, ns, actions, vars, local_scope);
                Statement::Let { name: new_name, value: new_val }
            } else {
                local_scope.insert(name.clone());
                let new_val = qualify_expr(value, ns, actions, vars, local_scope);
                Statement::Let { name, value: new_val }
            }
        }
        Statement::State { name, value } => {
            if local_scope.is_empty() {
                let new_name = format!("{}.{}", ns, name);
                let new_val = qualify_expr(value, ns, actions, vars, local_scope);
                Statement::State { name: new_name, value: new_val }
            } else {
                local_scope.insert(name.clone());
                let new_val = qualify_expr(value, ns, actions, vars, local_scope);
                Statement::State { name, value: new_val }
            }
        }
        Statement::Assignment { name, value } => {
            let new_name = if !local_scope.contains(&name) && vars.contains(&name) {
                format!("{}.{}", ns, name)
            } else {
                name
            };
            let new_val = qualify_expr(value, ns, actions, vars, local_scope);
            Statement::Assignment { name: new_name, value: new_val }
        }
        Statement::SubscriptAssignment { object, key, value } => Statement::SubscriptAssignment {
            object: qualify_expr(object, ns, actions, vars, local_scope),
            key: qualify_expr(key, ns, actions, vars, local_scope),
            value: qualify_expr(value, ns, actions, vars, local_scope),
        },
        Statement::While { cond, body } => {
            let new_cond = qualify_expr(cond, ns, actions, vars, local_scope);
            let new_body = body.into_iter().map(|s| qualify_stmt(s, ns, actions, vars, local_scope)).collect();
            Statement::While { cond: new_cond, body: new_body }
        }
        Statement::If { cond, then_branch, else_branch } => {
            let new_cond = qualify_expr(cond, ns, actions, vars, local_scope);
            let new_then = then_branch.into_iter().map(|s| qualify_stmt(s, ns, actions, vars, local_scope)).collect();
            let new_else = else_branch.into_iter().map(|s| qualify_stmt(s, ns, actions, vars, local_scope)).collect();
            Statement::If { cond: new_cond, then_branch: new_then, else_branch: new_else }
        }
        Statement::Print(val) => Statement::Print(qualify_expr(val, ns, actions, vars, local_scope)),
        Statement::Return(Some(val)) => Statement::Return(Some(qualify_expr(val, ns, actions, vars, local_scope))),
        Statement::Expr(val) => Statement::Expr(qualify_expr(val, ns, actions, vars, local_scope)),
        Statement::Respond(val) => Statement::Respond(qualify_expr(val, ns, actions, vars, local_scope)),
        other => other,
    }
}

fn apply_namespace(statements: Vec<parser::Statement>, ns: &str) -> Vec<parser::Statement> {
    use parser::Statement;
    let mod_actions: HashSet<String> = statements.iter().filter_map(|s| {
        if let Statement::Action { name, .. } = s {
            Some(name.clone())
        } else {
            None
        }
    }).collect();

    let mod_vars: HashSet<String> = statements.iter().filter_map(|s| {
        match s {
            Statement::Let { name, .. } | Statement::State { name, .. } => Some(name.clone()),
            _ => None,
        }
    }).collect();

    let mut global_scope = HashSet::new();
    statements.into_iter().map(|s| qualify_stmt(s, ns, &mod_actions, &mod_vars, &mut global_scope)).collect()
}

fn expand_file(
    path: &Path,
    alias: Option<String>,
    visiting: &mut HashSet<PathBuf>,
    loaded: &mut HashSet<(PathBuf, Option<String>)>,
) -> Result<Vec<parser::Statement>, String> {
    let canonical = fs::canonicalize(path).map_err(|e| format!("cannot resolve module '{}': {}", path.display(), e))?;
    if visiting.contains(&canonical) {
        return Err(format!("cyclic import detected at '{}'", canonical.display()));
    }
    if !loaded.insert((canonical.clone(), alias.clone())) { return Ok(Vec::new()); }
    visiting.insert(canonical.clone());
    let source = fs::read_to_string(&canonical).map_err(|e| format!("cannot read module '{}': {}", canonical.display(), e))?;
    let statements = parse_source(&source).map_err(|e| format!("{}: {}", canonical.display(), e))?;
    let mut expanded = Vec::new();
    for statement in statements {
        if let parser::Statement::Import { path: import_path, alias: sub_alias } = statement {
            let requested = Path::new(&import_path);
            if requested.is_absolute() {
                return Err(format!("{}: imports must use relative paths", canonical.display()));
            }
            let mut resolved = canonical.parent().unwrap_or(Path::new(".")).join(requested);
            if resolved.extension().is_none() { resolved.set_extension("aayu"); }
            if !resolved.exists() {
                let mut curr = canonical.parent();
                let mut found = None;
                while let Some(parent) = curr {
                    let mut candidate = parent.join(requested);
                    if candidate.extension().is_none() { candidate.set_extension("aayu"); }
                    if candidate.exists() {
                        found = Some(candidate);
                        break;
                    }
                    let mut std_candidate = parent.join("std").join(requested);
                    if std_candidate.extension().is_none() { std_candidate.set_extension("aayu"); }
                    if std_candidate.exists() {
                        found = Some(std_candidate);
                        break;
                    }
                    curr = parent.parent();
                }
                if let Some(f) = found {
                    resolved = f;
                } else {
                    let mut global_dirs: Vec<PathBuf> = Vec::new();
                    if let Ok(cwd) = std::env::current_dir() {
                        global_dirs.push(cwd.clone());
                        let mut p = cwd.as_path();
                        while let Some(parent) = p.parent() {
                            global_dirs.push(parent.to_path_buf());
                            p = parent;
                        }
                    }
                    if let Ok(exe) = std::env::current_exe() {
                        if let Some(p) = exe.parent() {
                            global_dirs.push(p.to_path_buf());
                            if let Some(pp) = p.parent() {
                                global_dirs.push(pp.to_path_buf());
                                if let Some(ppp) = pp.parent() {
                                    global_dirs.push(ppp.to_path_buf());
                                }
                            }
                        }
                    }
                    if let Ok(home) = std::env::var("AAYU_HOME") {
                        global_dirs.push(PathBuf::from(home));
                    }
                    if let Ok(userprofile) = std::env::var("USERPROFILE") {
                        global_dirs.push(PathBuf::from(userprofile).join(".aayu"));
                    }
                    if let Ok(home) = std::env::var("HOME") {
                        global_dirs.push(PathBuf::from(home).join(".aayu"));
                    }
                    for gdir in &global_dirs {
                        let mut cand1 = gdir.join(requested);
                        if cand1.extension().is_none() { cand1.set_extension("aayu"); }
                        if cand1.exists() { resolved = cand1; break; }

                        let mut cand2 = gdir.join("std").join(requested);
                        if cand2.extension().is_none() { cand2.set_extension("aayu"); }
                        if cand2.exists() { resolved = cand2; break; }

                        if requested.starts_with("std") {
                            if let Ok(stripped) = requested.strip_prefix("std") {
                                let mut cand3 = gdir.join("std").join(stripped);
                                if cand3.extension().is_none() { cand3.set_extension("aayu"); }
                                if cand3.exists() { resolved = cand3; break; }
                            }
                        }
                    }
                }
            }
            expanded.extend(expand_file(&resolved, sub_alias, visiting, loaded)?);
        } else {
            expanded.push(statement);
        }
    }
    visiting.remove(&canonical);
    if let Some(ref ns) = alias {
        expanded = apply_namespace(expanded, ns);
    }
    Ok(expanded)
}

fn validate_supported(statements: &[parser::Statement]) -> Result<(), String> {
    fn check_expr(expr: &parser::Expr, functions: &HashMap<String, usize>) -> Result<(), String> {
        use parser::Expr;
        match expr {
            Expr::Integer(_) => Ok(()),
            Expr::Call { callee, args } => {
                if callee == "len" {
                    if args.len() != 1 { return Err("len() expects exactly one argument".into()); }
                } else if !matches!(callee.as_str(),
                    "print" | "respond" | "hash_str" | "type" | "typeof" | "platform" | "arch" | "env_get" |
                    "read_file" | "write_file" | "time_micros" | "time_us" | "time_ms" | "file_mtime" | "file_size" | "sleep_ms" | "os_exec" | "sys_command" | "append" | "push" |
                    "dict_keys" | "keys" | "str_contains" |
                    "tensor" | "tensor_create" | "tensor_zeros" | "tensor_ones" | "tensor_randn" |
                    "tensor_matmul" | "tensor_add" | "tensor_sub" | "tensor_mul" | "tensor_div" |
                    "tensor_scale" | "tensor_relu" | "tensor_gelu" | "tensor_sigmoid" |
                    "tensor_tanh" | "tensor_silu" | "tensor_softmax" | "tensor_layernorm" |
                    "tensor_attention" | "tensor_transpose" | "tensor_reshape" |
                    "tensor_to_array" | "tensor_shape" | "tensor_sum" | "tensor_mean" |
                    "tensor_norm" | "tensor_argmax" | "cosine_similarity" |
                    "math_exp" | "math_ln" | "math_sqrt" | "math_pow" | "math_sin" |
                    "math_cos" | "math_tan" | "math_tanh" |
                    "math_floor" | "math_ceil" | "math_round" | "math_abs" |
                    "crypto_sha256" | "crypto_sha512" | "crypto_hmac_sha256" | "crypto_timing_safe_eq" | "crypto_secure_compare" |
                    "crypto_random_hex" | "crypto_aes_gcm_encrypt" | "crypto_aes_gcm_decrypt" |
                    "db_btree_open" | "db_btree_set" | "db_btree_get" | "db_btree_count" | "db_btree_flush"
                ) {

                    let expected = functions.get(callee).ok_or_else(|| format!("unknown function '{}'", callee))?;
                    if args.len() != *expected {
                        return Err(format!("function '{}' expects {} arguments, got {}", callee, expected, args.len()));
                    }
                }
                for arg in args { check_expr(arg, functions)?; }
                Ok(())
            }
            Expr::Unary { expr, .. } => check_expr(expr, functions),
            Expr::Array(items) => { for item in items { check_expr(item, functions)?; } Ok(()) },
            Expr::Dict(entries) => { for (key, value) in entries { check_expr(key, functions)?; check_expr(value, functions)?; } Ok(()) },
            Expr::Index { object, key } => { check_expr(object, functions)?; check_expr(key, functions) },
            Expr::Binary { left, right, .. } => { check_expr(left, functions)?; check_expr(right, functions) },
            _ => Ok(()),
        }
    }

    fn collect_expr_vars(expr: &parser::Expr, vars: &mut HashSet<String>) {
        match expr {
            parser::Expr::Variable(name) => { vars.insert(name.clone()); }
            parser::Expr::Unary { expr, .. } => collect_expr_vars(expr, vars),
            parser::Expr::Array(items) => for item in items { collect_expr_vars(item, vars); },
            parser::Expr::Dict(entries) => for (key, value) in entries { collect_expr_vars(key, vars); collect_expr_vars(value, vars); },
            parser::Expr::Index { object, key } => { collect_expr_vars(object, vars); collect_expr_vars(key, vars); }
            parser::Expr::Binary { left, right, .. } => { collect_expr_vars(left, vars); collect_expr_vars(right, vars); }
            parser::Expr::Call { args, .. } => for arg in args { collect_expr_vars(arg, vars); },
            _ => {}
        }
    }
    fn collect_body_vars(body: &[parser::Statement], vars: &mut HashSet<String>) {
        use parser::Statement;
        for statement in body {
            match statement {
                Statement::Let { name, value } | Statement::Assignment { name, value } => {
                    vars.insert(name.clone()); collect_expr_vars(value, vars);
                }
                Statement::SubscriptAssignment { object, key, value } => {
                    collect_expr_vars(object, vars);
                    collect_expr_vars(key, vars);
                    collect_expr_vars(value, vars);
                }
                Statement::Print(value) | Statement::State { value, .. } |
                Statement::Return(Some(value)) | Statement::Expr(value) |
                Statement::Respond(value) => collect_expr_vars(value, vars),
                Statement::While { cond, body } => { collect_expr_vars(cond, vars); collect_body_vars(body, vars); }
                Statement::If { cond, then_branch, else_branch } => {
                    collect_expr_vars(cond, vars); collect_body_vars(then_branch, vars); collect_body_vars(else_branch, vars);
                }
                Statement::Page { body, .. } | Statement::Widget { children: body, .. } => collect_body_vars(body, vars),
                Statement::Route { handlers, .. } => {
                    for (_, h_body) in handlers {
                        collect_body_vars(h_body, vars);
                    }
                }
                _ => {}
            }
        }
    }

    let mut functions = HashMap::new();
    for statement in statements {
        if let parser::Statement::Action { name, args, .. } = statement {
            if functions.insert(name.clone(), args.len()).is_some() {
                return Err(format!("duplicate function declaration '{}'", name));
            }
            let mut seen = HashSet::new();
            if args.iter().any(|arg| !seen.insert(arg)) {
                return Err(format!("function '{}' has duplicate parameter names", name));
            }
            if args.len() > 256 { return Err(format!("function '{}' exceeds the 256-parameter limit", name)); }
        }
    }

    let mut global_vars = HashSet::new();
    for statement in statements {
        if let parser::Statement::Action { name, args, body } = statement {
            let mut vars: HashSet<String> = args.iter().cloned().collect();
            collect_body_vars(body, &mut vars);
            if vars.len() > 256 { return Err(format!("function '{}' requires {} local slots; maximum is 256", name, vars.len())); }
        } else {
            collect_body_vars(std::slice::from_ref(statement), &mut global_vars);
        }
    }
    if global_vars.len() > 256 { return Err(format!("program requires {} local slots; maximum is 256", global_vars.len())); }

    fn validate_body(statements: &[parser::Statement], functions: &HashMap<String, usize>, in_function: bool, loop_depth: usize) -> Result<(), String> {
        for statement in statements {
            use parser::Statement;
            match statement {
                Statement::Action { body, .. } => {
                    if in_function { return Err("nested action declarations are not supported".into()); }
                    validate_body(body, functions, true, 0)?;
                }
                Statement::Let { value, .. } | Statement::Assignment { value, .. } |
                Statement::Print(value) | Statement::State { value, .. } => check_expr(value, functions)?,
                Statement::While { cond, body } => { check_expr(cond, functions)?; validate_body(body, functions, in_function, loop_depth + 1)?; },
                Statement::If { cond, then_branch, else_branch } => {
                    check_expr(cond, functions)?;
                    validate_body(then_branch, functions, in_function, loop_depth)?;
                    validate_body(else_branch, functions, in_function, loop_depth)?;
                }
                Statement::Break => {
                    if loop_depth == 0 { return Err("break is only valid inside a while loop".into()); }
                }
                Statement::Continue => {
                    if loop_depth == 0 { return Err("continue is only valid inside a while loop".into()); }
                }
                Statement::Return(Some(value)) => {
                    if !in_function { return Err("return is only valid inside an action".into()); }
                    check_expr(value, functions)?;
                }
                Statement::Return(None) if !in_function => return Err("return is only valid inside an action".into()),
                Statement::Expr(value) => check_expr(value, functions)?,
                Statement::SubscriptAssignment { object, key, value } => {
                    check_expr(object, functions)?;
                    check_expr(key, functions)?;
                    check_expr(value, functions)?;
                }
                Statement::Respond(value) => check_expr(value, functions)?,
                Statement::Route { handlers, .. } => {
                    for (_, h_body) in handlers {
                        validate_body(h_body, functions, in_function, loop_depth)?;
                    }
                }
                Statement::Model { .. } => {}
                Statement::Page { body, .. } | Statement::Widget { children: body, .. } => validate_body(body, functions, in_function, loop_depth)?,
                _ => {}
            }
        }
        Ok(())
    }

    validate_body(statements, &functions, false, 0)
}
