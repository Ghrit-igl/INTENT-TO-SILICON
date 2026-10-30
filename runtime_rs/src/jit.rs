use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::vm::NanVal;

pub fn can_compile(bytecode: &[u8], constants: &[NanVal]) -> bool {
    let len = bytecode.len();
    let mut ip = 0;
    while ip < len {
        let op = bytecode[ip];
        ip += 1;
        match op {
            0x00 | 0xFF | 0x02 | 81 => {}
            0x01 => {
                if ip + 2 > len { return false; }
                let idx = ((bytecode[ip] as usize) << 8) | bytecode[ip + 1] as usize;
                ip += 2;
                if idx >= constants.len() { return false; }
                let c = constants[idx];
                if !c.is_int() && !c.is_float() && !c.is_bool() && !c.is_null() && !c.is_string() {
                    return false;
                }
            }
            0x05 | 0x06 | 0x07 | 0x08 | 23..=26 | 46..=47 => {
                if ip + 1 > len { return false; }
                ip += 1;
            }
            16..=20 | 38..=44 | 58..=60 => {}
            27..=30 | 45 => {
                if ip + 3 > len { return false; }
                ip += 3;
            }
            31 | 37 => {
                if ip + 5 > len { return false; }
                ip += 5;
            }
            32 | 33 => {
                if ip + 2 > len { return false; }
                ip += 2;
            }
            _ => return false,
        }
    }
    true
}

pub fn compile_and_run(bytecode: &[u8], constants: &[NanVal], strings: &[String]) -> bool {
    let c_code = generate_c(bytecode, constants, strings, false);
    
    let source_path = "aayu_jit_temp.c";
    let exe_path = if cfg!(windows) { "aayu_jit_temp.exe" } else { "./aayu_jit_temp" };
    let run_path = if cfg!(windows) { ".\\aayu_jit_temp.exe" } else { "./aayu_jit_temp" };
    
    {
        let mut file = match File::create(source_path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        if file.write_all(c_code.as_bytes()).is_err() || file.flush().is_err() {
            let _ = std::fs::remove_file(source_path);
            return false;
        }
    }
    
    let mut compiler = Command::new("gcc");
    compiler.args(["-O2", "-march=native", source_path, "-o", exe_path]);
    
    let mut compiled = false;
    if let Ok(s) = compiler.status() {
        if s.success() {
            compiled = true;
        }
    }

    if !compiled {
        let mut clang = Command::new("clang");
        clang.args(["-O2", "-march=native", source_path, "-o", exe_path]);
        if let Ok(s) = clang.status() {
            if s.success() {
                compiled = true;
            }
        }
    }

    if !compiled {
        let _ = std::fs::remove_file(source_path);
        return false;
    }
    
    let run_res = Command::new(run_path).status();
    let _ = std::fs::remove_file(source_path);
    let _ = std::fs::remove_file(exe_path);

    match run_res {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

pub fn compile_and_run_bench(bytecode: &[u8], constants: &[NanVal]) -> bool {
    let c_code = generate_c(bytecode, constants, &[], true);
    
    let source_path = "aayu_jit_bench_temp.c";
    let exe_path = if cfg!(windows) { "aayu_jit_bench_temp.exe" } else { "./aayu_jit_bench_temp" };
    let run_path = if cfg!(windows) { ".\\aayu_jit_bench_temp.exe" } else { "./aayu_jit_bench_temp" };
    
    {
        let mut file = match File::create(source_path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        if file.write_all(c_code.as_bytes()).is_err() || file.flush().is_err() {
            let _ = std::fs::remove_file(source_path);
            return false;
        }
    }
    
    let mut compiler = Command::new("gcc");
    compiler.args(["-O2", "-march=native", source_path, "-o", exe_path]);
    
    let mut compiled = false;
    if let Ok(s) = compiler.status() {
        if s.success() {
            compiled = true;
        }
    }

    if !compiled {
        let _ = std::fs::remove_file(source_path);
        return false;
    }
    
    let run_res = Command::new(run_path).status();
    let _ = std::fs::remove_file(source_path);
    let _ = std::fs::remove_file(exe_path);

    match run_res {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

pub fn build_native_binary(bytecode: &[u8], constants: &[NanVal], strings: &[String], output_path: &str) -> Result<(), String> {
    let c_code = generate_c(bytecode, constants, strings, false);
    let source_path = format!("{}.c", output_path);
    
    {
        let mut file = File::create(&source_path).map_err(|e| format!("Failed to create C source: {}", e))?;
        file.write_all(c_code.as_bytes()).map_err(|e| format!("Failed to write C source: {}", e))?;
        file.flush().map_err(|e| format!("Failed to flush C source: {}", e))?;
    }
    
    let mut compiler = Command::new("gcc");
    compiler.args(["-O2", "-march=native", &source_path, "-o", output_path]);
    let status = compiler.status().map_err(|e| format!("GCC execution failed: {}", e))?;
    
    let _ = std::fs::remove_file(&source_path);
    if status.success() {
        Ok(())
    } else {
        Err("GCC compilation failed with non-zero exit code".into())
    }
}

fn generate_c(bytecode: &[u8], constants: &[NanVal], strings: &[String], benchmark_timer: bool) -> String {
    let mut c = String::new();
    c.push_str("#include <stdio.h>\n");
    c.push_str("#include <stdint.h>\n");
    c.push_str("#include <stdbool.h>\n");
    c.push_str("#include <string.h>\n");
    c.push_str("#ifdef _WIN32\n");
    c.push_str("#include <windows.h>\n");
    c.push_str("#else\n");
    c.push_str("#include <time.h>\n");
    c.push_str("#endif\n\n");

    // NanVal definitions
    c.push_str("#define TAG_INT    0x7FF8000000000000ULL\n");
    c.push_str("#define TAG_BOOL   0x7FFA000000000000ULL\n");
    c.push_str("#define TAG_NULL   0x7FFB000000000000ULL\n");
    c.push_str("#define TAG_STR    0x7FFC000000000000ULL\n");
    c.push_str("#define TAG_MASK   0xFFFF000000000000ULL\n");
    c.push_str("#define PAYLOAD    0x0000FFFFFFFFFFFFULL\n\n");

    c.push_str("static inline bool is_int(uint64_t v) { return (v & TAG_MASK) == TAG_INT; }\n");
    c.push_str("static inline bool is_bool(uint64_t v) { return (v & TAG_MASK) == TAG_BOOL; }\n");
    c.push_str("static inline bool is_null(uint64_t v) { return (v & TAG_MASK) == TAG_NULL; }\n");
    c.push_str("static inline bool is_str(uint64_t v) { return (v & TAG_MASK) == TAG_STR; }\n");
    c.push_str("static inline bool is_float(uint64_t v) {\n");
    c.push_str("    uint64_t tag = v & TAG_MASK;\n");
    c.push_str("    return tag != TAG_INT && tag != TAG_BOOL && tag != TAG_NULL && tag != TAG_STR;\n");
    c.push_str("}\n\n");

    c.push_str("static inline int64_t as_int(uint64_t v) {\n");
    c.push_str("    return (int64_t)(((int64_t)((v & PAYLOAD) << 16)) >> 16);\n");
    c.push_str("}\n");

    c.push_str("static inline double as_float(uint64_t v) {\n");
    c.push_str("    union { uint64_t u; double d; } u; u.u = v; return u.d;\n");
    c.push_str("}\n");

    c.push_str("static inline bool as_bool(uint64_t v) {\n");
    c.push_str("    if (is_bool(v)) return (v & 1) != 0;\n");
    c.push_str("    if (is_null(v)) return false;\n");
    c.push_str("    if (is_int(v)) return as_int(v) != 0;\n");
    c.push_str("    if (is_float(v)) { double f = as_float(v); return f != 0.0; }\n");
    c.push_str("    return true;\n");
    c.push_str("}\n\n");

    c.push_str("static inline uint64_t box_int(int64_t v) { return TAG_INT | ((uint64_t)v & PAYLOAD); }\n");
    c.push_str("static inline uint64_t box_float(double f) { union { double d; uint64_t u; } u; u.d = f; return u.u; }\n");
    c.push_str("static inline uint64_t box_bool(bool b) { return TAG_BOOL | (b ? 1 : 0); }\n\n");

    c.push_str("static inline uint64_t val_inc(uint64_t v) {\n");
    c.push_str("    if (is_int(v)) return box_int(as_int(v) + 1);\n");
    c.push_str("    if (is_float(v)) return box_float(as_float(v) + 1.0);\n");
    c.push_str("    return box_int(as_int(v) + 1);\n");
    c.push_str("}\n");

    c.push_str("static inline uint64_t val_dec(uint64_t v) {\n");
    c.push_str("    if (is_int(v)) return box_int(as_int(v) - 1);\n");
    c.push_str("    if (is_float(v)) return box_float(as_float(v) - 1.0);\n");
    c.push_str("    return box_int(as_int(v) - 1);\n");
    c.push_str("}\n");

    c.push_str("static inline uint64_t val_add(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return box_int(as_int(a) + as_int(b));\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return box_float(fa + fb);\n");
    c.push_str("    }\n");
    c.push_str("    return box_int(as_int(a) + as_int(b));\n");
    c.push_str("}\n");

    c.push_str("static inline uint64_t val_sub(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return box_int(as_int(a) - as_int(b));\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return box_float(fa - fb);\n");
    c.push_str("    }\n");
    c.push_str("    return box_int(as_int(a) - as_int(b));\n");
    c.push_str("}\n");

    c.push_str("static inline uint64_t val_mul(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return box_int(as_int(a) * as_int(b));\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return box_float(fa * fb);\n");
    c.push_str("    }\n");
    c.push_str("    return box_int(as_int(a) * as_int(b));\n");
    c.push_str("}\n");

    c.push_str("static inline uint64_t val_div(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) {\n");
    c.push_str("        int64_t ib = as_int(b);\n");
    c.push_str("        return box_int(ib != 0 ? as_int(a) / ib : 0);\n");
    c.push_str("    }\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return box_float(fb != 0.0 ? fa / fb : 0.0);\n");
    c.push_str("    }\n");
    c.push_str("    int64_t ib = as_int(b);\n");
    c.push_str("    return box_int(ib != 0 ? as_int(a) / ib : 0);\n");
    c.push_str("}\n");

    c.push_str("static inline uint64_t val_mod(uint64_t a, uint64_t b) {\n");
    c.push_str("    int64_t ib = as_int(b);\n");
    c.push_str("    return box_int(ib != 0 ? as_int(a) % ib : 0);\n");
    c.push_str("}\n");

    c.push_str("static inline bool val_eq(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return as_int(a) == as_int(b);\n");
    c.push_str("    if (is_float(a) && is_float(b)) return as_float(a) == as_float(b);\n");
    c.push_str("    return a == b;\n");
    c.push_str("}\n");

    c.push_str("static inline bool val_lt(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return as_int(a) < as_int(b);\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return fa < fb;\n");
    c.push_str("    }\n");
    c.push_str("    return as_int(a) < as_int(b);\n");
    c.push_str("}\n");

    c.push_str("static inline bool val_gt(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return as_int(a) > as_int(b);\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return fa > fb;\n");
    c.push_str("    }\n");
    c.push_str("    return as_int(a) > as_int(b);\n");
    c.push_str("}\n");

    c.push_str("static inline bool val_lte(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return as_int(a) <= as_int(b);\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return fa <= fb;\n");
    c.push_str("    }\n");
    c.push_str("    return as_int(a) <= as_int(b);\n");
    c.push_str("}\n");

    c.push_str("static inline bool val_gte(uint64_t a, uint64_t b) {\n");
    c.push_str("    if (is_int(a) && is_int(b)) return as_int(a) >= as_int(b);\n");
    c.push_str("    if (is_float(a) || is_float(b)) {\n");
    c.push_str("        double fa = is_float(a) ? as_float(a) : (double)as_int(a);\n");
    c.push_str("        double fb = is_float(b) ? as_float(b) : (double)as_int(b);\n");
    c.push_str("        return fa >= fb;\n");
    c.push_str("    }\n");
    c.push_str("    return as_int(a) >= as_int(b);\n");
    c.push_str("}\n\n");

    // String Pool Table
    c.push_str("static const char* string_pool[] = {\n");
    for s in strings {
        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r");
        c.push_str(&format!("    \"{}\",\n", escaped));
    }
    c.push_str("    \"\"\n};\n");
    c.push_str(&format!("static const size_t string_pool_count = {};\n\n", strings.len()));

    c.push_str("static void print_val(uint64_t v) {\n");
    c.push_str("    if (is_str(v)) {\n");
    c.push_str("        size_t idx = (size_t)(v & PAYLOAD);\n");
    c.push_str("        if (idx < string_pool_count) { printf(\"%s\\n\", string_pool[idx]); }\n");
    c.push_str("    } else if (is_int(v)) {\n");
    c.push_str("        printf(\"%lld\\n\", (long long)as_int(v));\n");
    c.push_str("    } else if (is_bool(v)) {\n");
    c.push_str("        printf(\"%s\\n\", (v & 1) ? \"true\" : \"false\");\n");
    c.push_str("    } else if (is_float(v)) {\n");
    c.push_str("        printf(\"%g\\n\", as_float(v));\n");
    c.push_str("    } else {\n");
    c.push_str("        printf(\"%lld\\n\", (long long)as_int(v));\n");
    c.push_str("    }\n");
    c.push_str("}\n\n");

    c.push_str("int main(int argc, char** argv) {\n");
    c.push_str("    uint64_t stack[4096];\n");
    c.push_str("    int sp = 0;\n");
    c.push_str("    uint64_t locals[256] = {0};\n");
    c.push_str("    uint64_t globals[256] = {0};\n\n");

    if benchmark_timer {
        c.push_str("#ifdef _WIN32\n");
        c.push_str("    LARGE_INTEGER freq, start, end;\n");
        c.push_str("    QueryPerformanceFrequency(&freq);\n");
        c.push_str("    QueryPerformanceCounter(&start);\n");
        c.push_str("#else\n");
        c.push_str("    struct timespec start, end;\n");
        c.push_str("    clock_gettime(CLOCK_MONOTONIC, &start);\n");
        c.push_str("#endif\n\n");
    }

    let mut ip = 0;
    while ip < bytecode.len() {
        let op_ip = ip;
        let op = bytecode[ip];
        c.push_str(&format!("  L_{}:\n", op_ip));
        ip += 1;
        
        match op {
            0x00 | 0xFF => { c.push_str("    goto L_END;\n"); }
            0x01 => {
                let idx = ((bytecode[ip] as usize) << 8) | bytecode[ip + 1] as usize;
                ip += 2;
                let bits = if idx < constants.len() { constants[idx].0 } else { 0 };
                c.push_str(&format!("    stack[sp++] = 0x{:016X}ULL;\n", bits));
            }
            0x02 => {
                c.push_str("    if (sp > 0) sp--;\n");
            }
            0x05 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    stack[sp++] = locals[{}];\n", idx));
            }
            0x06 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    locals[{}] = stack[--sp];\n", idx));
            }
            0x07 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    stack[sp++] = globals[{}];\n", idx));
            }
            0x08 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    globals[{}] = stack[--sp];\n", idx));
            }
            16 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = val_add(a, b); }\n");
            }
            17 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = val_sub(a, b); }\n");
            }
            18 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = val_mul(a, b); }\n");
            }
            19 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = val_div(a, b); }\n");
            }
            20 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = val_mod(a, b); }\n");
            }
            23 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    locals[{}] = val_inc(locals[{}]);\n", idx, idx));
            }
            24 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    locals[{}] = val_dec(locals[{}]);\n", idx, idx));
            }
            25 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    globals[{}] = val_inc(globals[{}]);\n", idx, idx));
            }
            26 => {
                let idx = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    globals[{}] = val_dec(globals[{}]);\n", idx, idx));
            }
            27 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                ip += 3;
                let bits = if const_idx < constants.len() { constants[const_idx].0 } else { 0 };
                c.push_str(&format!("    locals[{}] = val_add(locals[{}], 0x{:016X}ULL);\n", slot, slot, bits));
            }
            28 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                ip += 3;
                let bits = if const_idx < constants.len() { constants[const_idx].0 } else { 0 };
                c.push_str(&format!("    globals[{}] = val_add(globals[{}], 0x{:016X}ULL);\n", slot, slot, bits));
            }
            29 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                ip += 3;
                let bits = if const_idx < constants.len() { constants[const_idx].0 } else { 0 };
                c.push_str(&format!("    locals[{}] = val_sub(locals[{}], 0x{:016X}ULL);\n", slot, slot, bits));
            }
            30 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                ip += 3;
                let bits = if const_idx < constants.len() { constants[const_idx].0 } else { 0 };
                c.push_str(&format!("    globals[{}] = val_sub(globals[{}], 0x{:016X}ULL);\n", slot, slot, bits));
            }
            31 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                let target = ((bytecode[ip + 3] as usize) << 8) | bytecode[ip + 4] as usize;
                ip += 5;
                let limit = if const_idx < constants.len() { constants[const_idx].as_int() } else { 0 };
                c.push_str(&format!("    if (as_int(globals[{}]) >= {}LL) goto L_{};\n", slot, limit, target));
            }
            37 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                let target = ((bytecode[ip + 3] as usize) << 8) | bytecode[ip + 4] as usize;
                ip += 5;
                let limit = if const_idx < constants.len() { constants[const_idx].as_int() } else { 0 };
                c.push_str(&format!("    if (as_int(locals[{}]) >= {}LL) goto L_{};\n", slot, limit, target));
            }
            32 => {
                let offset = ((bytecode[ip] as usize) << 8) | bytecode[ip + 1] as usize;
                ip += 2;
                c.push_str(&format!("    goto L_{};\n", offset));
            }
            33 => {
                let offset = ((bytecode[ip] as usize) << 8) | bytecode[ip + 1] as usize;
                ip += 2;
                c.push_str(&format!("    if (!as_bool(stack[--sp])) goto L_{};\n", offset));
            }
            38 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(val_eq(a, b)); }\n");
            }
            39 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(!val_eq(a, b)); }\n");
            }
            41 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(val_lt(a, b)); }\n");
            }
            42 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(val_gt(a, b)); }\n");
            }
            43 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(val_lte(a, b)); }\n");
            }
            44 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(val_gte(a, b)); }\n");
            }
            45 => {
                let slot = bytecode[ip] as usize;
                let const_idx = ((bytecode[ip + 1] as usize) << 8) | bytecode[ip + 2] as usize;
                ip += 3;
                let bits = if const_idx < constants.len() { constants[const_idx].0 } else { 0 };
                c.push_str(&format!("    stack[sp++] = val_sub(locals[{}], 0x{:016X}ULL);\n", slot, bits));
            }
            46 => {
                let slot = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    locals[{}] = val_add(locals[{}], stack[--sp]);\n", slot, slot));
            }
            47 => {
                let slot = bytecode[ip] as usize;
                ip += 1;
                c.push_str(&format!("    globals[{}] = val_add(globals[{}], stack[--sp]);\n", slot, slot));
            }
            58 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(as_bool(a) && as_bool(b)); }\n");
            }
            59 => {
                c.push_str("    { uint64_t b = stack[--sp]; uint64_t a = stack[--sp]; stack[sp++] = box_bool(as_bool(a) || as_bool(b)); }\n");
            }
            60 => {
                c.push_str("    { uint64_t a = stack[--sp]; stack[sp++] = box_bool(!as_bool(a)); }\n");
            }
            81 => {
                c.push_str("    print_val(stack[--sp]);\n");
            }
            _ => {
                c.push_str(&format!("    /* Unimplemented op {} */\n", op));
            }
        }
    }
    
    c.push_str("  L_END:\n");
    if benchmark_timer {
        c.push_str("#ifdef _WIN32\n");
        c.push_str("    QueryPerformanceCounter(&end);\n");
        c.push_str("    double ns = ((double)(end.QuadPart - start.QuadPart) * 1000000000.0) / freq.QuadPart;\n");
        c.push_str("    printf(\"AAYU_VM_NS %.0f\\n\", ns);\n");
        c.push_str("#else\n");
        c.push_str("    clock_gettime(CLOCK_MONOTONIC, &end);\n");
        c.push_str("    double ns = (end.tv_sec - start.tv_sec) * 1e9 + (end.tv_nsec - start.tv_nsec);\n");
        c.push_str("    printf(\"AAYU_VM_NS %.0f\\n\", ns);\n");
        c.push_str("#endif\n");
    }

    c.push_str("    return 0;\n}\n");
    c
}
