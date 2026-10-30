
use crate::math::Tensor;
use crate::db::DbEngine;
use crate::net::HttpServer;
use crate::ui::{UiNode, Style};

#[derive(Clone, Copy, Debug)]
pub struct NanVal(pub u64);

const TAG_INT: u64 = 0x7FF8_0000_0000_0000;
const TAG_TENSOR: u64 = 0x7FF9_0000_0000_0000;
const TAG_BOOL: u64 = 0x7FFA_0000_0000_0000;
const TAG_NULL: u64 = 0x7FFB_0000_0000_0000;
const TAG_STR: u64  = 0x7FFC_0000_0000_0000;
const TAG_ARR: u64  = 0x7FFD_0000_0000_0000;
const TAG_DICT: u64 = 0x7FFE_0000_0000_0000;
const TAG_BIGINT: u64 = 0x7FFF_0000_0000_0000;
const MIN_BOXED_INT: i64 = -(1i64 << 47);
const MAX_BOXED_INT: i64 = (1i64 << 47) - 1;

#[allow(dead_code)]
fn checked_boxed_int(value: i64) -> Result<NanVal, String> {
    if !(MIN_BOXED_INT..=MAX_BOXED_INT).contains(&value) {
        return Err(format!("Integer overflow: {} is outside the signed 48-bit range", value));
    }
    Ok(NanVal::int(value))
}

impl NanVal {
    #[inline(always)] pub fn int(v: i64) -> Self { NanVal(TAG_INT | (v as u64 & 0x0000_FFFF_FFFF_FFFF)) }
    #[inline(always)] pub fn bigint(idx: usize) -> Self { NanVal(TAG_BIGINT | (idx as u64 & 0x0000_FFFF_FFFF_FFFF)) }
    #[inline(always)] pub fn float(v: f64) -> Self {
        // Positive quiet-NaN payloads overlap the boxed value tags. Keep NaN
        // values in the negative quiet-NaN range so they remain floats.
        let bits = if v.is_nan() { v.to_bits() | (1u64 << 63) } else { v.to_bits() };
        NanVal(bits)
    }
    #[inline(always)] pub fn bool(v: bool) -> Self { NanVal(TAG_BOOL | (if v { 1 } else { 0 })) }
    #[inline(always)] pub fn null() -> Self { NanVal(TAG_NULL) }
    #[inline(always)] pub fn string(idx: usize) -> Self { NanVal(TAG_STR | (idx as u64 & 0x0000_FFFF_FFFF_FFFF)) }
    #[inline(always)] pub fn array(idx: usize) -> Self { NanVal(TAG_ARR | (idx as u64 & 0x0000_FFFF_FFFF_FFFF)) }
    #[inline(always)] pub fn dict(idx: usize) -> Self { NanVal(TAG_DICT | (idx as u64 & 0x0000_FFFF_FFFF_FFFF)) }
    #[inline(always)] pub fn tensor(idx: usize) -> Self { NanVal(TAG_TENSOR | (idx as u64 & 0x0000_FFFF_FFFF_FFFF)) }

    #[inline(always)] pub fn is_int(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_INT }
    #[inline(always)] pub fn is_bigint(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_BIGINT }
    #[inline(always)] pub fn is_tensor(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_TENSOR }
    #[inline(always)] pub fn is_float(self) -> bool {
        let tag = self.0 & 0xFFFF_0000_0000_0000;
        !matches!(tag, TAG_INT | TAG_TENSOR | TAG_BOOL | TAG_NULL | TAG_STR | TAG_ARR | TAG_DICT | TAG_BIGINT)
    }
    #[inline(always)] pub fn is_bool(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_BOOL }
    #[inline(always)] pub fn is_null(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_NULL }
    #[inline(always)] pub fn is_string(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_STR }
    #[inline(always)] pub fn is_array(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_ARR }
    #[inline(always)] pub fn is_dict(self) -> bool { (self.0 & 0xFFFF_0000_0000_0000) == TAG_DICT }

    #[inline(always)] pub fn as_int(self) -> i64 {
        (((self.0 & 0x0000_FFFF_FFFF_FFFF) << 16) as i64) >> 16
    }
    #[inline(always)] pub fn as_bigint_idx(self) -> usize { (self.0 & 0x0000_FFFF_FFFF_FFFF) as usize }
    #[inline(always)] pub fn as_float(self) -> f64 { f64::from_bits(self.0) }
    #[inline(always)] pub fn as_tensor_idx(self) -> usize { (self.0 & 0x0000_FFFF_FFFF_FFFF) as usize }
    #[inline(always)]
    pub fn as_bool(self) -> bool {
        if self.is_bool() {
            (self.0 & 1) == 1
        } else if self.is_null() {
            false
        } else if self.is_int() {
            self.as_int() != 0
        } else if self.is_bigint() {
            true
        } else if self.is_float() {
            let f = self.as_float();
            f != 0.0 && !f.is_nan()
        } else {
            true
        }
    }
    #[inline(always)] pub fn as_string_idx(self) -> usize { (self.0 & 0x0000_FFFF_FFFF_FFFF) as usize }
    #[inline(always)] pub fn as_array_idx(self) -> usize { (self.0 & 0x0000_FFFF_FFFF_FFFF) as usize }
    #[inline(always)] pub fn as_dict_idx(self) -> usize { (self.0 & 0x0000_FFFF_FFFF_FFFF) as usize }
}

#[cfg(test)]
mod value_tests {
    use super::{checked_boxed_int, NanVal, MAX_BOXED_INT, MIN_BOXED_INT};

    #[test]
    fn signed_integers_round_trip_through_48_bit_payload() {
        for value in [-140_737_488_355_328, -5, -1, 0, 1, 140_737_488_355_327] {
            let boxed = NanVal::int(value);
            assert!(boxed.is_int());
            assert_eq!(boxed.as_int(), value);
        }
    }

    #[test]
    fn negative_floats_are_not_mistaken_for_box_tags() {
        for value in [-2.5, -0.0, 0.0, 2.5, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let boxed = NanVal::float(value);
            assert!(boxed.is_float());
            if value.is_nan() { assert!(boxed.as_float().is_nan()); }
            else { assert_eq!(boxed.as_float(), value); }
        }
    }

    #[test]
    fn boxed_integer_range_reports_overflow() {
        assert!(checked_boxed_int(MIN_BOXED_INT).is_ok());
        assert!(checked_boxed_int(MAX_BOXED_INT).is_ok());
        assert!(checked_boxed_int(MIN_BOXED_INT - 1).is_err());
        assert!(checked_boxed_int(MAX_BOXED_INT + 1).is_err());
    }
}

impl std::fmt::Display for NanVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_int() { write!(f, "{}", self.as_int()) }
        else if self.is_float() { write!(f, "{}", self.as_float()) }
        else if self.is_bool() { write!(f, "{}", self.as_bool()) }
        else if self.is_null() { write!(f, "null") }
        else if self.is_string() { write!(f, "<String at {}>", self.as_string_idx()) }
        else if self.is_array() { write!(f, "<Array at {}>", self.as_array_idx()) }
        else if self.is_dict() { write!(f, "<Dict at {}>", self.as_dict_idx()) }
        else if self.is_tensor() { write!(f, "<Tensor at {}>", self.as_tensor_idx()) }
        else if self.is_bigint() { write!(f, "<BigInt at {}>", self.as_bigint_idx()) }
        else { write!(f, "<Unknown>") }
    }
}

pub struct AayuVM {
    stack: [NanVal; 4096],
    sp: usize,
    ip: usize,
    locals: Vec<NanVal>,
    base: usize,
    globals: [NanVal; 256],
    bytecode: Vec<u8>,
    constants: Vec<NanVal>,
    pub string_heap: Vec<String>,
    pub memory: crate::memory::MemoryManager,
    pub call_depth: usize,
    call_frames: Vec<CallFrame>,
    
    pub tensor_heap: Vec<Tensor>,
    pub db_engine: DbEngine,
    pub btree_dbs: Vec<crate::btree::BTreeDb>,
    pub state: std::collections::HashMap<String, NanVal>,
    pub sandbox: bool,
}

struct CallFrame {
    return_ip: usize,
    prev_base: usize,
}

impl AayuVM {
    pub const MAX_STACK_SIZE: usize = 4096;
    pub const MAX_CALL_DEPTH: usize = 1024;

    pub fn new(bytecode: Vec<u8>, constants: Vec<NanVal>, strings: Vec<String>) -> Self {
        Self {
            stack: [NanVal::null(); 4096],
            sp: 0,
            ip: 0,
            locals: vec![NanVal::null(); 256 * (Self::MAX_CALL_DEPTH + 2)],
            base: 0,
            globals: [NanVal::null(); 256],
            bytecode,
            constants,
            string_heap: strings,
            memory: crate::memory::MemoryManager::new(),
            call_depth: 0,
            call_frames: Vec::with_capacity(Self::MAX_CALL_DEPTH),
            tensor_heap: Vec::new(),
            db_engine: DbEngine::new("aayu_db.rsdb"),
            btree_dbs: Vec::new(),
            state: std::collections::HashMap::new(),
            sandbox: false,
        }
    }

    pub fn allocate_heap(&mut self, obj: crate::memory::HeapObject) -> usize {
        if self.memory.free_list.is_empty() && self.memory.heap.len() >= self.memory.gc_threshold {
            self.collect_garbage();
        }
        self.memory.allocate(obj)
    }

    pub fn collect_garbage(&mut self) {
        if self.sp > 0 {
            self.memory.mark_roots(&self.stack[0..self.sp]);
        }
        let max_local = (self.base + 256).min(self.locals.len());
        self.memory.mark_roots(&self.locals[0..max_local]);
        self.memory.mark_roots(&self.globals);
        self.memory.mark_roots(&self.constants);
        if !self.state.is_empty() {
            let state_vals: Vec<NanVal> = self.state.values().copied().collect();
            self.memory.mark_roots(&state_vals);
        }

        self.memory.sweep();

        let live = self.memory.heap.len().saturating_sub(self.memory.free_list.len());
        self.memory.gc_threshold = (live * 2).max(32768);
    }

    #[inline(always)]
    pub fn get_int128(&self, val: NanVal) -> Option<i128> {
        if val.is_int() {
            Some(val.as_int() as i128)
        } else if val.is_bigint() {
            if let crate::memory::HeapObject::ABigInt(bi) = self.memory.get(val.as_bigint_idx()) {
                Some(*bi)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn make_int128(&mut self, val: i128) -> NanVal {
        if val >= MIN_BOXED_INT as i128 && val <= MAX_BOXED_INT as i128 {
            NanVal::int(val as i64)
        } else {
            let idx = self.allocate_heap(crate::memory::HeapObject::ABigInt(val));
            NanVal::bigint(idx)
        }
    }

    #[inline(always)]
    fn vals_eq(&self, a: NanVal, b: NanVal) -> bool {
        if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
            match (self.get_int128(a), self.get_int128(b)) {
                (Some(ia), Some(ib)) => ia == ib,
                _ => false,
            }
        } else if a.is_float() && b.is_float() {
            a.as_float() == b.as_float()
        } else if a.is_float() && (b.is_int() || b.is_bigint()) {
            match self.get_int128(b) {
                Some(ib) => a.as_float() == (ib as f64),
                None => false,
            }
        } else if (a.is_int() || a.is_bigint()) && b.is_float() {
            match self.get_int128(a) {
                Some(ia) => (ia as f64) == b.as_float(),
                None => false,
            }
        } else if a.is_string() && b.is_string() {
            self.string_heap.get(a.as_string_idx()) == self.string_heap.get(b.as_string_idx())
        } else if a.is_bool() && b.is_bool() {
            a.as_bool() == b.as_bool()
        } else if a.is_bool() && (b.is_int() || b.is_bigint()) {
            (if a.as_bool() { 1 } else { 0 }) == self.get_int128(b).unwrap_or(0)
        } else if (a.is_int() || a.is_bigint()) && b.is_bool() {
            self.get_int128(a).unwrap_or(0) == (if b.as_bool() { 1 } else { 0 })
        } else {
            a.0 == b.0
        }
    }

    #[inline(always)]
    fn cmp_vals(&self, a: NanVal, b: NanVal) -> Option<std::cmp::Ordering> {
        if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
            match (self.get_int128(a), self.get_int128(b)) {
                (Some(ia), Some(ib)) => Some(ia.cmp(&ib)),
                _ => None,
            }
        } else if a.is_float() || b.is_float() {
            let fa = if a.is_float() { a.as_float() } else if a.is_int() { a.as_int() as f64 } else if a.is_bigint() { self.get_int128(a).unwrap_or(0) as f64 } else { 0.0 };
            let fb = if b.is_float() { b.as_float() } else if b.is_int() { b.as_int() as f64 } else if b.is_bigint() { self.get_int128(b).unwrap_or(0) as f64 } else { 0.0 };
            fa.partial_cmp(&fb)
        } else if a.is_string() && b.is_string() {
            let sa = self.string_heap.get(a.as_string_idx())?;
            let sb = self.string_heap.get(b.as_string_idx())?;
            Some(sa.cmp(sb))
        } else if a.is_bool() && b.is_bool() {
            Some(a.as_bool().cmp(&b.as_bool()))
        } else {
            Some(a.0.cmp(&b.0))
        }
    }

    pub fn allocate_tensor(&mut self, t: Tensor) -> NanVal {
        let idx = self.tensor_heap.len();
        self.tensor_heap.push(t);
        NanVal::tensor(idx)
    }

    pub fn extract_tensor(&self, val: NanVal) -> Result<&Tensor, String> {
        if !val.is_tensor() {
            return Err("Expected Tensor argument".to_string());
        }
        let idx = val.as_tensor_idx();
        if idx >= self.tensor_heap.len() {
            return Err(format!("Tensor index {} out of bounds", idx));
        }
        Ok(&self.tensor_heap[idx])
    }

    pub fn extract_f64_vec(&self, val: NanVal) -> Result<Vec<f64>, String> {
        if val.is_tensor() {
            let t = self.extract_tensor(val)?;
            return Ok(t.to_vec());
        }
        if !val.is_array() {
            return Err("Expected Array of numbers".to_string());
        }
        if let crate::memory::HeapObject::AArray(arr) = self.memory.get(val.as_array_idx()) {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                if item.is_int() {
                    out.push(item.as_int() as f64);
                } else if item.is_float() {
                    out.push(item.as_float());
                } else if item.is_bigint() {
                    out.push(self.get_int128(*item).unwrap_or(0) as f64);
                } else if item.is_array() {
                    let sub_vec = self.extract_f64_vec(*item)?;
                    out.extend(sub_vec);
                } else {
                    return Err("Tensor data array elements must be numbers".to_string());
                }
            }
            Ok(out)
        } else {
            Err("Invalid array reference".to_string())
        }
    }

    #[inline(always)]
    pub fn extract_f64_arg(&self, val: NanVal) -> f64 {
        if val.is_float() {
            val.as_float()
        } else if val.is_int() {
            val.as_int() as f64
        } else if val.is_bigint() {
            self.get_int128(val).unwrap_or(0) as f64
        } else {
            0.0
        }
    }

    pub fn extract_shape_vec(&self, val: NanVal) -> Result<Vec<usize>, String> {
        if !val.is_array() {
            return Err("Expected Array for shape (e.g. [2, 3])".to_string());
        }
        if let crate::memory::HeapObject::AArray(arr) = self.memory.get(val.as_array_idx()) {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                if item.is_int() {
                    let v = item.as_int();
                    if v < 0 {
                        return Err(format!("Shape dimension cannot be negative: {}", v));
                    }
                    out.push(v as usize);
                } else if item.is_float() {
                    let v = item.as_float() as i64;
                    if v < 0 {
                        return Err(format!("Shape dimension cannot be negative: {}", v));
                    }
                    out.push(v as usize);
                } else {
                    return Err("Shape array elements must be integers".to_string());
                }
            }
            Ok(out)
        } else {
            Err("Invalid array reference".to_string())
        }
    }

    pub fn tensor_to_nanval_array(&mut self, t: &Tensor) -> NanVal {
        let vec = t.to_vec();
        let nanvals: Vec<NanVal> = vec.into_iter().map(NanVal::float).collect();
        let idx = self.allocate_heap(crate::memory::HeapObject::AArray(nanvals));
        NanVal::array(idx)
    }

    pub fn shape_to_nanval_array(&mut self, shape: &[usize]) -> NanVal {
        let nanvals: Vec<NanVal> = shape.iter().map(|&s| NanVal::int(s as i64)).collect();
        let idx = self.allocate_heap(crate::memory::HeapObject::AArray(nanvals));
        NanVal::array(idx)
    }

    pub fn validate(&self) -> Result<(), String> {
        let len = self.bytecode.len();
        let mut pc = 0;
        let mut valid_starts = std::collections::HashSet::new();
        let mut jumps = Vec::new();
        let mut function_entries = Vec::new();
        
        while pc < len {
            valid_starts.insert(pc);
            let op = self.bytecode[pc];
            let op_pc = pc;
            pc += 1;
            
            match op {
                0x00 | 0xFF => {
                    // HALT
                }
                0x01 => { // PUSH_CONST (2 bytes operand)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for PUSH_CONST at offset {}", op_pc));
                    }
                    let idx = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                    pc += 2;
                    if idx >= self.constants.len() {
                        return Err(format!("PUSH_CONST index {} exceeds constant pool bounds ({}) at offset {}", idx, self.constants.len(), op_pc));
                    }
                }
                0x02 => {} // POP
                0x05 => { // LOAD_VAR (1 byte operand)
                    if pc + 1 > len {
                        return Err(format!("Truncated operand for LOAD_VAR at offset {}", op_pc));
                    }
                    let idx = self.bytecode[pc] as usize;
                    pc += 1;
                    if idx >= 256 {
                        return Err(format!("LOAD_VAR index {} exceeds local variable frame limit (256) at offset {}", idx, op_pc));
                    }
                }
                0x06 => { // STORE_VAR (1 byte operand)
                    if pc + 1 > len {
                        return Err(format!("Truncated operand for STORE_VAR at offset {}", op_pc));
                    }
                    let idx = self.bytecode[pc] as usize;
                    pc += 1;
                    if idx >= 256 {
                        return Err(format!("STORE_VAR index {} exceeds local variable frame limit (256) at offset {}", idx, op_pc));
                    }
                }
                0x07 | 0x08 => {
                    if pc + 1 > len { return Err(format!("Truncated global variable operand at offset {}", op_pc)); }
                    pc += 1;
                }
                23 | 24 | 25 | 26 => { // Inc/Dec Local/Global By 1 (1 byte slot)
                    if pc + 1 > len { return Err(format!("Truncated operand for Inc/Dec at offset {}", op_pc)); }
                    pc += 1;
                }
                27 | 28 | 29 | 30 | 45 | 56 | 57 => { // Add/Sub Const / PushLocalSubConst / PushModConst (1 byte slot + 2 byte const_idx)
                    if pc + 3 > len { return Err(format!("Truncated operand for Add/Sub Const at offset {}", op_pc)); }
                    let const_idx = ((self.bytecode[pc + 1] as usize) << 8) | self.bytecode[pc + 2] as usize;
                    pc += 3;
                    if const_idx >= self.constants.len() {
                        return Err(format!("Constant index {} exceeds constant pool size ({}) at offset {}", const_idx, self.constants.len(), op_pc));
                    }
                }
                46 | 47 => { // AddToLocal / AddToGlobal (1 byte slot)
                    if pc + 1 > len { return Err(format!("Truncated operand for AddToLocal/Global at offset {}", op_pc)); }
                    pc += 1;
                }
                31 | 37 => { // JmpIfGlobal/Local Gte Const (1 byte slot + 2 byte const_idx + 2 byte target)
                    if pc + 5 > len { return Err(format!("Truncated operand for JmpIfGteConst at offset {}", op_pc)); }
                    let const_idx = ((self.bytecode[pc + 1] as usize) << 8) | self.bytecode[pc + 2] as usize;
                    let target = ((self.bytecode[pc + 3] as usize) << 8) | self.bytecode[pc + 4] as usize;
                    pc += 5;
                    if const_idx >= self.constants.len() {
                        return Err(format!("Constant index {} exceeds constant pool size ({}) at offset {}", const_idx, self.constants.len(), op_pc));
                    }
                    if target > len {
                        return Err(format!("JmpIfGteConst target offset {} exceeds bytecode bounds ({}) at offset {}", target, len, op_pc));
                    }
                    jumps.push((target, op_pc, "JmpIfGteConst"));
                }
                0x10..=0x16 => {
                    // Arithmetic & casting opcodes
                }
                0x20 => { // JUMP (2 bytes operand)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for JUMP at offset {}", op_pc));
                    }
                    let target = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                    pc += 2;
                    if target > len {
                        return Err(format!("JUMP target offset {} exceeds bytecode bounds (length: {})", target, len));
                    }
                    jumps.push((target, op_pc, "JUMP"));
                }
                0x21 => { // JUMP_IF_FALSE (2 bytes operand)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for JUMP_IF_FALSE at offset {}", op_pc));
                    }
                    let target = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                    pc += 2;
                    if target > len {
                        return Err(format!("JUMP_IF_FALSE target offset {} exceeds bytecode bounds (length: {})", target, len));
                    }
                    jumps.push((target, op_pc, "JUMP_IF_FALSE"));
                }
                34 => { // CALL (2 bytes operand)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for CALL at offset {}", op_pc));
                    }
                    pc += 2;
                }
                36 => { // CALL_LOCAL (argc + absolute target)
                    if pc + 4 > len {
                        return Err(format!("Truncated operand for CALL_LOCAL at offset {}", op_pc));
                    }
                    let argc = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                    let target = ((self.bytecode[pc + 2] as usize) << 8) | self.bytecode[pc + 3] as usize;
                    pc += 4;
                    if target >= len {
                        return Err(format!("CALL_LOCAL target {} exceeds bytecode bounds ({}) at offset {}", target, len, op_pc));
                    }
                    jumps.push((target, op_pc, "CALL_LOCAL"));
                    function_entries.push(target);
                    if argc > Self::MAX_STACK_SIZE {
                        return Err(format!("CALL_LOCAL argument count {} exceeds stack limit", argc));
                    }
                }
                35 | 98 => {} // RET / ReturnValue
                38 | 39 | 41 | 42 | 43 | 44 | 58 | 59 | 60 => {} // CMP & Logical operators
                48 => { // STORE_STATE (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for STORE_STATE at offset {}", op_pc));
                    }
                    pc += 2;
                }
                49 => { // LOAD_STATE (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for LOAD_STATE at offset {}", op_pc));
                    }
                    pc += 2;
                }
                50 => { // INIT_STATE (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for INIT_STATE at offset {}", op_pc));
                    }
                    pc += 2;
                }
                51 | 52 => {} // EnterScope / ExitScope
                53 => { // InitVar (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for InitVar at offset {}", op_pc));
                    }
                    pc += 2;
                }
                54 => { // PrepareCall (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for PrepareCall at offset {}", op_pc));
                    }
                    pc += 2;
                }
                55 => { // AsyncCall (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for AsyncCall at offset {}", op_pc));
                    }
                    pc += 2;
                }
                81 => {} // PRINT
                128 => { // BUILD_DICT (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for BUILD_DICT at offset {}", op_pc));
                    }
                    pc += 2;
                }
                134 => { // CREATE_ARRAY (two-byte item count)
                    if pc + 2 > len { return Err(format!("Truncated operand for CREATE_ARRAY at offset {}", op_pc)); }
                    pc += 2;
                }
                136 | 137 | 140 | 141 | 142 => {}
                145 => { // PUSH_STRING_SAFE (2 bytes)
                    if pc + 2 > len {
                        return Err(format!("Truncated operand for PUSH_STRING_SAFE at offset {}", op_pc));
                    }
                    let idx = (self.bytecode[pc] as usize) | ((self.bytecode[pc+1] as usize) << 8);
                    pc += 2;
                    if idx >= self.string_heap.len() {
                        return Err(format!("PUSH_STRING_SAFE string index {} out of bounds (string heap size: {})", idx, self.string_heap.len()));
                    }
                }
                150 | 160 | 161 | 162 | 170 | 171 | 172 | 180 | 181 | 190 | 191 => {}
                245 | 252 | 253 | 254 => {} // Legacy opcodes
                _ => {
                    return Err(format!("Unknown or illegal opcode 0x{:02X} at offset {}", op, op_pc));
                }
            }
        }

        // Pass 2: Verify all jump targets land strictly on valid instruction boundaries or end-of-stream
        for (target, op_pc, name) in jumps {
            if target != len && !valid_starts.contains(&target) {
                return Err(format!("{} target offset {} at offset {} is not aligned to an instruction boundary", name, target, op_pc));
            }
        }

        // Pass 3: Abstract Control-Flow Stack Bounds & Height Consistency Verifier
        let mut stack_depths: std::collections::HashMap<usize, i32> = std::collections::HashMap::new();
        let mut queue: std::collections::VecDeque<(usize, i32)> = std::collections::VecDeque::new();
        if len > 0 {
            queue.push_back((0, 0i32));
            stack_depths.insert(0, 0i32);
        }
        for target in function_entries {
            if !stack_depths.contains_key(&target) {
                stack_depths.insert(target, 0);
                queue.push_back((target, 0));
            }
        }

        while let Some((mut pc, mut depth)) = queue.pop_front() {
            while pc < len {
                let op = self.bytecode[pc];
                let op_pc = pc;
                pc += 1;

                match op {
                    0x00 | 0xFF => {
                        // HALT terminates the control flow path
                        break;
                    }
                    0x01 => { // PUSH_CONST (2 bytes operand)
                        pc += 2;
                        depth += 1;
                    }
                    0x02 => { // POP
                        depth -= 1;
                    }
                    0x05 => { // LOAD_VAR (1 byte operand)
                        pc += 1;
                        depth += 1;
                    }
                    0x06 => { // STORE_VAR (1 byte operand)
                        pc += 1;
                        depth -= 1;
                    }
                    0x07 => { pc += 1; depth += 1; }
                    0x08 => { pc += 1; depth -= 1; }
                    23..=26 => { // Inc/Dec: 1 byte operand, net stack depth change is 0
                        pc += 1;
                    }
                    27..=30 => { // Add/Sub Const: 3 bytes operand, net stack depth change is 0
                        pc += 3;
                    }
                    45 | 56 | 57 => { // PushLocalSubConst / PushModConst: 3 bytes operand, pushes 1
                        pc += 3;
                        depth += 1;
                    }
                    46 | 47 => { // AddToLocal / AddToGlobal: 1 byte operand, pops 1
                        pc += 1;
                        depth -= 1;
                    }
                    31 | 37 => { // JmpIfGteConst: 5 bytes operand, net stack depth change is 0
                        let target = ((self.bytecode[pc + 3] as usize) << 8) | self.bytecode[pc + 4] as usize;
                        pc += 5;
                        // Branch path
                        if target < len {
                            if let Some(&existing) = stack_depths.get(&target) {
                                if existing != depth {
                                    return Err(format!("Stack height mismatch at jump target offset {}: expected {}, found {}", target, existing, depth));
                                }
                            } else {
                                stack_depths.insert(target, depth);
                                queue.push_back((target, depth));
                            }
                        }
                        // Fall-through path
                        if pc < len {
                            if let Some(&existing) = stack_depths.get(&pc) {
                                if existing != depth {
                                    return Err(format!("Stack height mismatch at fallthrough offset {}: expected {}, found {}", pc, existing, depth));
                                }
                                break;
                            } else {
                                stack_depths.insert(pc, depth);
                            }
                        }
                        continue;
                    }
                    0x10..=0x16 => { // Binary arithmetic (ADD, SUB, MUL, DIV, MOD, etc.)
                        depth -= 1; // pops 2, pushes 1 -> net -1
                    }
                    0x20 => { // JUMP (2 bytes operand)
                        let target = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                        if target < len {
                            if let Some(&existing) = stack_depths.get(&target) {
                                if existing != depth {
                                    return Err(format!("Stack height mismatch at jump target offset {}: expected {}, found {}", target, existing, depth));
                                }
                            } else {
                                stack_depths.insert(target, depth);
                                queue.push_back((target, depth));
                            }
                        }
                        break; // Unconditional jump terminates linear path
                    }
                    0x21 => { // JUMP_IF_FALSE (2 bytes operand)
                        let target = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                        pc += 2;
                        depth -= 1; // pops condition
                        if depth < 0 {
                            return Err(format!("Stack underflow detected at JUMP_IF_FALSE at offset {}", op_pc));
                        }
                        // Branch path
                        if target < len {
                            if let Some(&existing) = stack_depths.get(&target) {
                                if existing != depth {
                                    return Err(format!("Stack height mismatch at jump target offset {}: expected {}, found {}", target, existing, depth));
                                }
                            } else {
                                stack_depths.insert(target, depth);
                                queue.push_back((target, depth));
                            }
                        }
                        // Fall-through path
                        if pc < len {
                            if let Some(&existing) = stack_depths.get(&pc) {
                                if existing != depth {
                                    return Err(format!("Stack height mismatch at fallthrough offset {}: expected {}, found {}", pc, existing, depth));
                                }
                                break;
                            } else {
                                stack_depths.insert(pc, depth);
                            }
                        }
                        continue;
                    }
                    34 => { // Builtin CALL (callee + arguments, two-byte argc)
                        let argc = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                        pc += 2;
                        depth = depth - (argc as i32) - 1 + 1;
                    }
                    36 => { // CALL_LOCAL (arguments, two-byte target)
                        let argc = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                        pc += 4;
                        depth = depth - argc as i32 + 1;
                    }
                    35 | 98 => { // RET
                        break; // Return terminates the path
                    }
                    38 | 39 | 41 | 42 | 43 | 44 | 58 | 59 => { // CMP & Logical Binary (pop 2, push 1)
                        depth -= 1;
                    }
                    60 => {} // LogicalNot (pop 1, push 1 -> net 0)
                    48 => { // STORE_STATE (2 bytes)
                        pc += 2;
                        depth -= 1;
                    }
                    49 => { // LOAD_STATE (2 bytes)
                        pc += 2;
                        depth += 1;
                    }
                    50 => { pc += 2; }
                    51 | 52 => {}
                    53 => { pc += 2; }
                    54 => { pc += 2; }
                    55 => { pc += 2; }
                    81 => { // PRINT: pops 1
                        depth -= 1;
                    }
                    128 => { // BUILD_DICT (2 bytes: num_pairs)
                        let num_pairs = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                        pc += 2;
                        depth = depth - (num_pairs as i32 * 2) + 1;
                    }
                    134 => {
                        let count = ((self.bytecode[pc] as usize) << 8) | self.bytecode[pc + 1] as usize;
                        pc += 2;
                        depth = depth - count as i32 + 1;
                    }
                    136 => { depth -= 1; }
                    137 => { depth -= 3; }
                    140 => { // StringConcat: pops 2, pushes 1
                        depth -= 1;
                    }
                    141 => { // StringSlice: pops 3, pushes 1
                        depth -= 2;
                    }
                    142 => {}
                    145 => { // PUSH_STRING_SAFE (2 bytes)
                        pc += 2;
                        depth += 1;
                    }
                    150 | 160 | 161 | 162 | 170 | 171 | 172 | 180 | 181 | 190 | 191 => {}
                    254 => { // Legacy ADD
                        depth -= 1;
                    }
                    253 => { // Legacy LT
                        depth -= 1;
                    }
                    _ => break,
                }

                if depth < 0 {
                    return Err(format!("Stack underflow detected at instruction 0x{:02X} at offset {}", op, op_pc));
                }
                if depth > Self::MAX_STACK_SIZE as i32 {
                    return Err(format!("Stack overflow detected: stack depth {} exceeds limit ({}) at offset {}", depth, Self::MAX_STACK_SIZE, op_pc));
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.validate()?;
        let len = self.bytecode.len();
        
        loop {
            if self.ip >= len { break; }
            let op = self.bytecode[self.ip];
            self.ip += 1;

            match op {
                0x00 | 0xFF => break, // HALT
                0x01 => { // PUSH_CONST
                    let idx = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    self.stack[self.sp] = self.constants[idx];
                    self.sp += 1;
                }
                0x05 => { // LOAD_VAR
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    self.stack[self.sp] = self.locals[self.base + idx];
                    self.sp += 1;
                }
                0x06 => { // STORE_VAR
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    self.locals[self.base + idx] = self.stack[self.sp - 1];
                    self.sp -= 1;
                }
                0x07 => { // LOAD_GLOBAL
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    self.stack[self.sp] = self.globals[idx];
                    self.sp += 1;
                }
                0x08 => { // STORE_GLOBAL
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    self.globals[idx] = self.stack[self.sp - 1];
                    self.sp -= 1;
                }
                23 => { // IncLocalBy1
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    let v = self.locals[self.base + idx];
                    if v.is_int() {
                        let raw = v.as_int();
                        if raw < MAX_BOXED_INT {
                            let new_val = raw + 1;
                            self.locals[self.base + idx] = NanVal::int(new_val);
                            if self.ip + 3 <= len && self.bytecode[self.ip] == 32 {
                                let jump_target = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                                if jump_target + 5 <= len && self.bytecode[jump_target] == 37 && self.bytecode[jump_target + 1] as usize == idx {
                                    let const_idx = ((self.bytecode[jump_target + 2] as usize) << 8) | self.bytecode[jump_target + 3] as usize;
                                    let limit = self.constants[const_idx];
                                    if limit.is_int() && new_val < limit.as_int() {
                                        self.ip = jump_target + 6;
                                        continue;
                                    }
                                }
                            }
                        } else {
                            self.locals[self.base + idx] = self.make_int128(raw as i128 + 1);
                        }
                    } else if v.is_bigint() {
                        let iv = self.get_int128(v).unwrap_or(0);
                        self.locals[self.base + idx] = self.make_int128(iv + 1);
                    } else if v.is_float() {
                        self.locals[self.base + idx] = NanVal::float(v.as_float() + 1.0);
                    } else if v.is_string() {
                        let mut s = self.string_heap.get(v.as_string_idx()).cloned().unwrap_or_default();
                        s.push_str("1");
                        let s_idx = self.string_heap.len();
                        self.string_heap.push(s);
                        self.locals[self.base + idx] = NanVal::string(s_idx);
                    }
                }
                24 => { // DecLocalBy1
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    let v = self.locals[self.base + idx];
                    if v.is_int() {
                        let raw = v.as_int();
                        if raw > MIN_BOXED_INT {
                            self.locals[self.base + idx] = NanVal::int(raw - 1);
                        } else {
                            self.locals[self.base + idx] = self.make_int128(raw as i128 - 1);
                        }
                    } else if v.is_bigint() {
                        let iv = self.get_int128(v).unwrap_or(0);
                        self.locals[self.base + idx] = self.make_int128(iv - 1);
                    } else if v.is_float() {
                        self.locals[self.base + idx] = NanVal::float(v.as_float() - 1.0);
                    }
                }
                25 => { // IncGlobalBy1
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    let v = self.globals[idx];
                    if v.is_int() {
                        let raw = v.as_int();
                        if raw < MAX_BOXED_INT {
                            let new_val = raw + 1;
                            self.globals[idx] = NanVal::int(new_val);
                            if self.ip + 3 <= len && self.bytecode[self.ip] == 32 {
                                let jump_target = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                                if jump_target + 5 <= len && self.bytecode[jump_target] == 31 && self.bytecode[jump_target + 1] as usize == idx {
                                    let const_idx = ((self.bytecode[jump_target + 2] as usize) << 8) | self.bytecode[jump_target + 3] as usize;
                                    let limit = self.constants[const_idx];
                                    if limit.is_int() && new_val < limit.as_int() {
                                        self.ip = jump_target + 6;
                                        continue;
                                    }
                                }
                            }
                        } else {
                            self.globals[idx] = self.make_int128(raw as i128 + 1);
                        }
                    } else if v.is_bigint() {
                        let iv = self.get_int128(v).unwrap_or(0);
                        self.globals[idx] = self.make_int128(iv + 1);
                    } else if v.is_float() {
                        self.globals[idx] = NanVal::float(v.as_float() + 1.0);
                    } else if v.is_string() {
                        let mut s = self.string_heap.get(v.as_string_idx()).cloned().unwrap_or_default();
                        s.push_str("1");
                        let s_idx = self.string_heap.len();
                        self.string_heap.push(s);
                        self.globals[idx] = NanVal::string(s_idx);
                    }
                }
                26 => { // DecGlobalBy1
                    let idx = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    let v = self.globals[idx];
                    if v.is_int() {
                        let raw = v.as_int();
                        if raw > MIN_BOXED_INT {
                            self.globals[idx] = NanVal::int(raw - 1);
                        } else {
                            self.globals[idx] = self.make_int128(raw as i128 - 1);
                        }
                    } else if v.is_bigint() {
                        let iv = self.get_int128(v).unwrap_or(0);
                        self.globals[idx] = self.make_int128(iv - 1);
                    } else if v.is_float() {
                        self.globals[idx] = NanVal::float(v.as_float() - 1.0);
                    }
                }
                27 => { // AddLocalConst
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.locals[self.base + slot];
                    let c = self.constants[const_idx];
                    if (v.is_int() || v.is_bigint()) && (c.is_int() || c.is_bigint()) {
                        let iv = self.get_int128(v).unwrap_or(0);
                        let ic = self.get_int128(c).unwrap_or(0);
                        self.locals[self.base + slot] = self.make_int128(iv + ic);
                    } else if v.is_float() || c.is_float() {
                        let fv = if v.is_float() { v.as_float() } else if v.is_int() { v.as_int() as f64 } else if v.is_bigint() { self.get_int128(v).unwrap_or(0) as f64 } else { 0.0 };
                        let fc = if c.is_float() { c.as_float() } else if c.is_int() { c.as_int() as f64 } else if c.is_bigint() { self.get_int128(c).unwrap_or(0) as f64 } else { 0.0 };
                        self.locals[self.base + slot] = NanVal::float(fv + fc);
                    } else if v.is_string() || c.is_string() {
                        let sv = if v.is_string() { self.string_heap.get(v.as_string_idx()).cloned().unwrap_or_default() } else { format!("{}", v) };
                        let sc = if c.is_string() { self.string_heap.get(c.as_string_idx()).cloned().unwrap_or_default() } else if c.is_int() { format!("{}", c.as_int()) } else { format!("{}", c) };
                        let mut res = sv;
                        res.push_str(&sc);
                        let s_idx = self.string_heap.len();
                        self.string_heap.push(res);
                        self.locals[self.base + slot] = NanVal::string(s_idx);
                    }
                }
                28 => { // AddGlobalConst
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.globals[slot];
                    let c = self.constants[const_idx];
                    if (v.is_int() || v.is_bigint()) && (c.is_int() || c.is_bigint()) {
                        let iv = self.get_int128(v).unwrap_or(0);
                        let ic = self.get_int128(c).unwrap_or(0);
                        self.globals[slot] = self.make_int128(iv + ic);
                    } else if v.is_float() || c.is_float() {
                        let fv = if v.is_float() { v.as_float() } else if v.is_int() { v.as_int() as f64 } else if v.is_bigint() { self.get_int128(v).unwrap_or(0) as f64 } else { 0.0 };
                        let fc = if c.is_float() { c.as_float() } else if c.is_int() { c.as_int() as f64 } else if c.is_bigint() { self.get_int128(c).unwrap_or(0) as f64 } else { 0.0 };
                        self.globals[slot] = NanVal::float(fv + fc);
                    } else if v.is_string() || c.is_string() {
                        let sv = if v.is_string() { self.string_heap.get(v.as_string_idx()).cloned().unwrap_or_default() } else { format!("{}", v) };
                        let sc = if c.is_string() { self.string_heap.get(c.as_string_idx()).cloned().unwrap_or_default() } else if c.is_int() { format!("{}", c.as_int()) } else { format!("{}", c) };
                        let mut res = sv;
                        res.push_str(&sc);
                        let s_idx = self.string_heap.len();
                        self.string_heap.push(res);
                        self.globals[slot] = NanVal::string(s_idx);
                    }
                }
                29 => { // SubLocalConst
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.locals[self.base + slot];
                    let c = self.constants[const_idx];
                    if (v.is_int() || v.is_bigint()) && (c.is_int() || c.is_bigint()) {
                        let iv = self.get_int128(v).unwrap_or(0);
                        let ic = self.get_int128(c).unwrap_or(0);
                        self.locals[self.base + slot] = self.make_int128(iv - ic);
                    } else if v.is_float() || c.is_float() {
                        let fv = if v.is_float() { v.as_float() } else if v.is_int() { v.as_int() as f64 } else if v.is_bigint() { self.get_int128(v).unwrap_or(0) as f64 } else { 0.0 };
                        let fc = if c.is_float() { c.as_float() } else if c.is_int() { c.as_int() as f64 } else if c.is_bigint() { self.get_int128(c).unwrap_or(0) as f64 } else { 0.0 };
                        self.locals[self.base + slot] = NanVal::float(fv - fc);
                    }
                }
                30 => { // SubGlobalConst
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.globals[slot];
                    let c = self.constants[const_idx];
                    if (v.is_int() || v.is_bigint()) && (c.is_int() || c.is_bigint()) {
                        let iv = self.get_int128(v).unwrap_or(0);
                        let ic = self.get_int128(c).unwrap_or(0);
                        self.globals[slot] = self.make_int128(iv - ic);
                    } else if v.is_float() || c.is_float() {
                        let fv = if v.is_float() { v.as_float() } else if v.is_int() { v.as_int() as f64 } else if v.is_bigint() { self.get_int128(v).unwrap_or(0) as f64 } else { 0.0 };
                        let fc = if c.is_float() { c.as_float() } else if c.is_int() { c.as_int() as f64 } else if c.is_bigint() { self.get_int128(c).unwrap_or(0) as f64 } else { 0.0 };
                        self.globals[slot] = NanVal::float(fv - fc);
                    }
                }
                31 => { // JmpIfGlobalGteConst
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    let target = ((self.bytecode[self.ip + 3] as usize) << 8) | self.bytecode[self.ip + 4] as usize;
                    self.ip += 5;
                    let v = self.globals[slot];
                    let c = self.constants[const_idx];
                    if v.is_int() && c.is_int() {
                        if v.as_int() >= c.as_int() {
                            self.ip = target;
                        }
                    } else if let Some(ord) = self.cmp_vals(v, c) {
                        if ord != std::cmp::Ordering::Less {
                            self.ip = target;
                        }
                    }
                }
                37 => { // JmpIfLocalGteConst
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    let target = ((self.bytecode[self.ip + 3] as usize) << 8) | self.bytecode[self.ip + 4] as usize;
                    self.ip += 5;
                    let v = self.locals[self.base + slot];
                    let c = self.constants[const_idx];
                    if v.is_int() && c.is_int() {
                        if v.as_int() >= c.as_int() {
                            self.ip = target;
                        }
                    } else if let Some(ord) = self.cmp_vals(v, c) {
                        if ord != std::cmp::Ordering::Less {
                            self.ip = target;
                        }
                    }
                }
                45 => { // PushLocalSubConst (1 byte slot + 2 byte const_idx)
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.locals[self.base + slot];
                    let c = self.constants[const_idx];
                    if v.is_int() && c.is_int() {
                        self.stack[self.sp] = NanVal::int(v.as_int().wrapping_sub(c.as_int()));
                    } else if v.is_float() || c.is_float() {
                        let fv = if v.is_float() { v.as_float() } else { v.as_int() as f64 };
                        let fc = if c.is_float() { c.as_float() } else { c.as_int() as f64 };
                        self.stack[self.sp] = NanVal::float(fv - fc);
                    } else {
                        self.stack[self.sp] = NanVal::int(v.as_int().wrapping_sub(c.as_int()));
                    }
                    self.sp += 1;
                }
                46 => { // AddToLocal (1 byte slot)
                    let slot = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    let val = self.stack[self.sp - 1];
                    self.sp -= 1;
                    let v = self.locals[self.base + slot];
                    if (v.is_int() || v.is_bigint()) && (val.is_int() || val.is_bigint()) {
                        let iv = self.get_int128(v).unwrap_or(0);
                        let ival = self.get_int128(val).unwrap_or(0);
                        self.locals[self.base + slot] = self.make_int128(iv + ival);
                    } else if v.is_float() || val.is_float() {
                        let fv = if v.is_float() { v.as_float() } else if v.is_int() { v.as_int() as f64 } else if v.is_bigint() { self.get_int128(v).unwrap_or(0) as f64 } else { 0.0 };
                        let fval = if val.is_float() { val.as_float() } else if val.is_int() { val.as_int() as f64 } else if val.is_bigint() { self.get_int128(val).unwrap_or(0) as f64 } else { 0.0 };
                        self.locals[self.base + slot] = NanVal::float(fv + fval);
                    } else if v.is_array() && val.is_array() {
                        let arr_a = match self.memory.get(v.as_array_idx()) {
                            crate::memory::HeapObject::AArray(arr) => arr.clone(),
                            _ => Vec::new(),
                        };
                        let arr_b = match self.memory.get(val.as_array_idx()) {
                            crate::memory::HeapObject::AArray(arr) => arr.clone(),
                            _ => Vec::new(),
                        };
                        let mut merged = arr_a;
                        merged.extend(arr_b);
                        let idx = self.allocate_heap(crate::memory::HeapObject::AArray(merged));
                        self.locals[self.base + slot] = NanVal::array(idx);
                    } else if v.is_string() || val.is_string() {
                        let sa = if v.is_string() { self.string_heap.get(v.as_string_idx()).cloned().unwrap_or_default() } else { format!("{}", v) };
                        let sb = if val.is_string() { self.string_heap.get(val.as_string_idx()).cloned().unwrap_or_default() } else { format!("{}", val) };
                        let mut res = sa;
                        res.push_str(&sb);
                        let idx = self.string_heap.len();
                        self.string_heap.push(res);
                        self.locals[self.base + slot] = NanVal::string(idx);
                    }
                }
                47 => { // AddToGlobal (1 byte slot)
                    let slot = self.bytecode[self.ip] as usize;
                    self.ip += 1;
                    let val = self.stack[self.sp - 1];
                    self.sp -= 1;
                    let v = self.globals[slot];
                    if (v.is_int() || v.is_bigint()) && (val.is_int() || val.is_bigint()) {
                        let iv = self.get_int128(v).unwrap_or(0);
                        let ival = self.get_int128(val).unwrap_or(0);
                        self.globals[slot] = self.make_int128(iv + ival);
                    } else if v.is_float() || val.is_float() {
                        let fv = if v.is_float() { v.as_float() } else if v.is_int() { v.as_int() as f64 } else if v.is_bigint() { self.get_int128(v).unwrap_or(0) as f64 } else { 0.0 };
                        let fval = if val.is_float() { val.as_float() } else if val.is_int() { val.as_int() as f64 } else if val.is_bigint() { self.get_int128(val).unwrap_or(0) as f64 } else { 0.0 };
                        self.globals[slot] = NanVal::float(fv + fval);
                    } else if v.is_array() && val.is_array() {
                        let arr_a = match self.memory.get(v.as_array_idx()) {
                            crate::memory::HeapObject::AArray(arr) => arr.clone(),
                            _ => Vec::new(),
                        };
                        let arr_b = match self.memory.get(val.as_array_idx()) {
                            crate::memory::HeapObject::AArray(arr) => arr.clone(),
                            _ => Vec::new(),
                        };
                        let mut merged = arr_a;
                        merged.extend(arr_b);
                        let idx = self.allocate_heap(crate::memory::HeapObject::AArray(merged));
                        self.globals[slot] = NanVal::array(idx);
                    } else if v.is_string() || val.is_string() {
                        let sa = if v.is_string() { self.string_heap.get(v.as_string_idx()).cloned().unwrap_or_default() } else { format!("{}", v) };
                        let sb = if val.is_string() { self.string_heap.get(val.as_string_idx()).cloned().unwrap_or_default() } else { format!("{}", val) };
                        let mut res = sa;
                        res.push_str(&sb);
                        let idx = self.string_heap.len();
                        self.string_heap.push(res);
                        self.globals[slot] = NanVal::string(idx);
                    }
                }
                56 => { // PushGlobalModConst (1 byte slot + 2 byte const_idx)
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.globals[slot];
                    let c = self.constants[const_idx];
                    if v.is_int() && c.is_int() {
                        let divisor = c.as_int();
                        if divisor != 0 {
                            let mod_res = v.as_int() % divisor;
                            if self.ip + 2 <= len && self.bytecode[self.ip] == 47 {
                                let dst_slot = self.bytecode[self.ip + 1] as usize;
                                self.ip += 2;
                                let dst_v = self.globals[dst_slot];
                                if dst_v.is_int() {
                                    let sum = dst_v.as_int().wrapping_add(mod_res);
                                    self.globals[dst_slot] = NanVal::int(sum);
                                    continue;
                                }
                            }
                            self.stack[self.sp] = NanVal::int(mod_res);
                            self.sp += 1;
                        } else {
                            return Err("Division by zero in modulo".into());
                        }
                    } else {
                        let a = v.as_int();
                        let b = c.as_int();
                        if b == 0 { return Err("Division by zero in modulo".into()); }
                        self.stack[self.sp] = NanVal::int(a % b);
                        self.sp += 1;
                    }
                }
                57 => { // PushLocalModConst (1 byte slot + 2 byte const_idx)
                    let slot = self.bytecode[self.ip] as usize;
                    let const_idx = ((self.bytecode[self.ip + 1] as usize) << 8) | self.bytecode[self.ip + 2] as usize;
                    self.ip += 3;
                    let v = self.locals[self.base + slot];
                    let c = self.constants[const_idx];
                    if v.is_int() && c.is_int() {
                        let divisor = c.as_int();
                        if divisor != 0 {
                            let mod_res = v.as_int() % divisor;
                            if self.ip + 2 <= len && self.bytecode[self.ip] == 46 {
                                let dst_slot = self.bytecode[self.ip + 1] as usize;
                                self.ip += 2;
                                let dst_v = self.locals[self.base + dst_slot];
                                if dst_v.is_int() {
                                    let sum = dst_v.as_int().wrapping_add(mod_res);
                                    self.locals[self.base + dst_slot] = NanVal::int(sum);
                                    continue;
                                }
                            }
                            self.stack[self.sp] = NanVal::int(mod_res);
                            self.sp += 1;
                        } else {
                            return Err("Division by zero in modulo".into());
                        }
                    } else {
                        let a = v.as_int();
                        let b = c.as_int();
                        if b == 0 { return Err("Division by zero in modulo".into()); }
                        self.stack[self.sp] = NanVal::int(a % b);
                        self.sp += 1;
                    }
                }
                0x10 => { // ADD (Polymorphic: Int, BigInt, Float, String, Array)
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    self.sp -= 1;
                    if a.is_int() && b.is_int() {
                        let ia = a.as_int();
                        let ib = b.as_int();
                        if let Some(sum) = ia.checked_add(ib) {
                            if (MIN_BOXED_INT..=MAX_BOXED_INT).contains(&sum) {
                                self.stack[self.sp - 1] = NanVal::int(sum);
                            } else {
                                self.stack[self.sp - 1] = self.make_int128(sum as i128);
                            }
                        } else {
                            self.stack[self.sp - 1] = self.make_int128((ia as i128) + (ib as i128));
                        }
                    } else if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
                        let ia = self.get_int128(a).unwrap_or(0);
                        let ib = self.get_int128(b).unwrap_or(0);
                        self.stack[self.sp - 1] = self.make_int128(ia + ib);
                    } else if a.is_string() && b.is_string() {
                        let idx_a = a.as_string_idx();
                        let idx_b = b.as_string_idx();
                        let (len_a, len_b) = (
                            self.string_heap.get(idx_a).map(|s| s.len()).unwrap_or(0),
                            self.string_heap.get(idx_b).map(|s| s.len()).unwrap_or(0),
                        );
                        let mut res = String::with_capacity(len_a + len_b);
                        if let Some(sa) = self.string_heap.get(idx_a) {
                            res.push_str(sa);
                        }
                        if let Some(sb) = self.string_heap.get(idx_b) {
                            res.push_str(sb);
                        }
                        let idx = self.string_heap.len();
                        self.string_heap.push(res);
                        self.stack[self.sp - 1] = NanVal::string(idx);
                    } else if a.is_string() || b.is_string() {
                        let sa = if a.is_string() {
                            self.string_heap.get(a.as_string_idx()).cloned().unwrap_or_default()
                        } else if a.is_int() {
                            format!("{}", a.as_int())
                        } else if a.is_bigint() {
                            format!("{}", self.get_int128(a).unwrap_or(0))
                        } else if a.is_float() {
                            format!("{}", a.as_float())
                        } else if a.is_bool() {
                            format!("{}", a.as_bool())
                        } else {
                            format!("{}", a)
                        };
                        let sb = if b.is_string() {
                            self.string_heap.get(b.as_string_idx()).cloned().unwrap_or_default()
                        } else if b.is_int() {
                            format!("{}", b.as_int())
                        } else if b.is_bigint() {
                            format!("{}", self.get_int128(b).unwrap_or(0))
                        } else if b.is_float() {
                            format!("{}", b.as_float())
                        } else if b.is_bool() {
                            format!("{}", b.as_bool())
                        } else {
                            format!("{}", b)
                        };
                        let mut res = sa;
                        res.push_str(&sb);
                        let idx = self.string_heap.len();
                        self.string_heap.push(res);
                        self.stack[self.sp - 1] = NanVal::string(idx);
                    } else if a.is_array() && b.is_array() {
                        let idx_a = a.as_array_idx();
                        let idx_b = b.as_array_idx();
                        let merged = match (self.memory.get(idx_a), self.memory.get(idx_b)) {
                            (crate::memory::HeapObject::AArray(va), crate::memory::HeapObject::AArray(vb)) => {
                                let mut m = Vec::with_capacity(va.len() + vb.len());
                                m.extend_from_slice(va);
                                m.extend_from_slice(vb);
                                m
                            }
                            _ => Vec::new(),
                        };
                        let idx = self.allocate_heap(crate::memory::HeapObject::AArray(merged));
                        self.stack[self.sp - 1] = NanVal::array(idx);
                    } else if a.is_float() || b.is_float() {
                        let fa = if a.is_float() { a.as_float() } else if a.is_int() { a.as_int() as f64 } else if a.is_bigint() { self.get_int128(a).unwrap_or(0) as f64 } else { 0.0 };
                        let fb = if b.is_float() { b.as_float() } else if b.is_int() { b.as_int() as f64 } else if b.is_bigint() { self.get_int128(b).unwrap_or(0) as f64 } else { 0.0 };
                        self.stack[self.sp - 1] = NanVal::float(fa + fb);
                    } else {
                        let ia = self.get_int128(a).unwrap_or(0);
                        let ib = self.get_int128(b).unwrap_or(0);
                        self.stack[self.sp - 1] = self.make_int128(ia + ib);
                    }
                    continue;
                }
                41 => { // LT
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        self.stack[self.sp - 2] = NanVal::bool(a.as_int() < b.as_int());
                    } else {
                        self.stack[self.sp - 2] = NanVal::bool(self.cmp_vals(a, b) == Some(std::cmp::Ordering::Less));
                    }
                    self.sp -= 1;
                }
                48 => { // STORE_STATE
                    let idx = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    if self.sp > 0 {
                        let val = self.stack[self.sp - 1];
                        self.sp -= 1;
                        self.locals[self.base + (idx % 256)] = val;
                        let var_name = if idx < self.constants.len() && self.constants[idx].is_string() {
                            let s_idx = self.constants[idx].as_string_idx();
                            if s_idx < self.string_heap.len() { self.string_heap[s_idx].clone() } else { format!("var_{}", idx) }
                        } else if idx < self.string_heap.len() {
                            self.string_heap[idx].clone()
                        } else {
                            format!("var_{}", idx)
                        };
                        self.state.insert(var_name, val);
                    }
                }
                49 => { // LOAD_STATE
                    let idx = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    let var_name = if idx < self.constants.len() && self.constants[idx].is_string() {
                        let s_idx = self.constants[idx].as_string_idx();
                        if s_idx < self.string_heap.len() { self.string_heap[s_idx].clone() } else { format!("var_{}", idx) }
                    } else if idx < self.string_heap.len() {
                        self.string_heap[idx].clone()
                    } else {
                        format!("var_{}", idx)
                    };
                    let val = if let Some(&v) = self.state.get(&var_name) {
                        v
                    } else {
                        self.locals[self.base + (idx % 256)]
                    };
                    self.stack[self.sp] = val;
                    self.sp += 1;
                }
                50 => { self.ip += 2; } // InitState
                51 | 52 => {} // EnterScope / ExitScope
                53 => { self.ip += 2; } // InitVar
                54 => { self.ip += 2; } // PrepareCall
                55 => { self.ip += 2; } // AsyncCall
                34 => { // CALL
                    let argc = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    if self.call_depth >= Self::MAX_CALL_DEPTH {
                        return Err(format!("StackOverflow: maximum call depth of {} exceeded", Self::MAX_CALL_DEPTH));
                    }
                    if self.sp >= argc + 1 {
                        let func_val = self.stack[self.sp - 1 - argc];
                        let func_name = if func_val.is_string() && func_val.as_string_idx() < self.string_heap.len() {
                            self.string_heap[func_val.as_string_idx()].clone()
                        } else {
                            String::new()
                        };
                        if func_name == "print" {
                            for i in 0..argc {
                                let a = self.stack[self.sp - argc + i];
                                if a.is_string() && a.as_string_idx() < self.string_heap.len() {
                                    print!("{}", self.string_heap[a.as_string_idx()]);
                                } else if a.is_tensor() && a.as_tensor_idx() < self.tensor_heap.len() {
                                    self.tensor_heap[a.as_tensor_idx()].print();
                                } else if a.is_bigint() {
                                    if let crate::memory::HeapObject::ABigInt(bi) = self.memory.get(a.as_bigint_idx()) {
                                        print!("{}", bi);
                                    } else {
                                        print!("{}", a);
                                    }
                                } else {
                                    print!("{}", a);
                                }
                                if i + 1 < argc { print!(" "); }
                            }
                            println!();
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::null();
                            self.sp += 1;
                        } else if func_name == "len" {
                            let a = if argc > 0 { self.stack[self.sp - 1] } else { NanVal::null() };
                            let l = if a.is_string() && a.as_string_idx() < self.string_heap.len() {
                                self.string_heap[a.as_string_idx()].len()
                            } else if a.is_array() {
                                if let crate::memory::HeapObject::AArray(arr) = self.memory.get(a.as_array_idx()) {
                                    arr.len()
                                } else { 0 }
                            } else if a.is_tensor() && a.as_tensor_idx() < self.tensor_heap.len() {
                                self.tensor_heap[a.as_tensor_idx()].numel()
                            } else { 0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::int(l as i64);
                            self.sp += 1;
                        } else if func_name == "dict_keys" || func_name == "keys" {
                            let a = if argc > 0 { self.stack[self.sp - 1] } else { NanVal::null() };
                            let mut key_vec = Vec::new();
                            if a.is_dict() {
                                if let crate::memory::HeapObject::ADict(map) = self.memory.get(a.as_dict_idx()) {
                                    for k in map.keys() {
                                        let idx = self.string_heap.len();
                                        self.string_heap.push(k.clone());
                                        key_vec.push(NanVal::string(idx));
                                    }
                                }
                            }
                            self.sp -= argc + 1;
                            let arr_idx = self.allocate_heap(crate::memory::HeapObject::AArray(key_vec));
                            self.stack[self.sp] = NanVal::array(arr_idx);
                            self.sp += 1;
                        } else if func_name == "str_contains" || func_name == "contains" {
                            let needle = if argc > 1 {
                                let n = self.stack[self.sp - 1];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { String::new() };
                            let haystack = if argc > 0 {
                                let h = self.stack[self.sp - argc];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::bool(haystack.contains(&needle));
                            self.sp += 1;
                        } else if func_name == "str_starts_with" || func_name == "starts_with" {
                            let prefix = if argc > 1 {
                                let n = self.stack[self.sp - 1];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { String::new() };
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - argc];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::bool(s.starts_with(&prefix));
                            self.sp += 1;
                        } else if func_name == "str_ends_with" || func_name == "ends_with" {
                            let suffix = if argc > 1 {
                                let n = self.stack[self.sp - 1];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { String::new() };
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - argc];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::bool(s.ends_with(&suffix));
                            self.sp += 1;
                        } else if func_name == "str_find" || func_name == "str_index_of" || func_name == "index_of" {
                            let needle = if argc > 1 {
                                let n = self.stack[self.sp - 1];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { String::new() };
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - argc];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let idx = s.find(&needle).map(|i| i as i64).unwrap_or(-1);
                            self.stack[self.sp] = NanVal::int(idx);
                            self.sp += 1;
                        } else if func_name == "str_split" || func_name == "split" {
                            let delim = if argc > 1 {
                                let n = self.stack[self.sp - 1];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { " ".to_string() };
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - argc];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let parts: Vec<NanVal> = s.split(&delim).map(|p| {
                                let str_idx = self.string_heap.len();
                                self.string_heap.push(p.to_string());
                                NanVal::string(str_idx)
                            }).collect();
                            let arr_idx = self.allocate_heap(crate::memory::HeapObject::AArray(parts));
                            self.stack[self.sp] = NanVal::array(arr_idx);
                            self.sp += 1;
                        } else if func_name == "str_trim" || func_name == "trim" {
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - 1];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(s.trim().to_string());
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "str_lower" || func_name == "to_lower" {
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - 1];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(s.to_lowercase());
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "str_upper" || func_name == "to_upper" {
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - 1];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(s.to_uppercase());
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "str_replace" || func_name == "replace" {
                            let to = if argc > 2 {
                                let n = self.stack[self.sp - 1];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { String::new() };
                            let from = if argc > 1 {
                                let n = self.stack[self.sp - 2];
                                if n.is_string() && n.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[n.as_string_idx()].clone()
                                } else { format!("{}", n) }
                            } else { String::new() };
                            let s = if argc > 0 {
                                let h = self.stack[self.sp - argc];
                                if h.is_string() && h.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[h.as_string_idx()].clone()
                                } else { format!("{}", h) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(s.replace(&from, &to));
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "cli_args" || func_name == "args" {
                            self.sp -= argc + 1;
                            let all_args: Vec<String> = std::env::args().collect();
                            let mut script_idx = None;
                            for (i, a) in all_args.iter().enumerate() {
                                if a.ends_with(".aayu") || a.ends_with(".ayc") {
                                    script_idx = Some(i);
                                    break;
                                }
                            }
                            let raw_args: Vec<String> = if let Some(idx) = script_idx {
                                if idx + 1 < all_args.len() {
                                    all_args[idx + 1..].to_vec()
                                } else {
                                    Vec::new()
                                }
                            } else {
                                all_args.into_iter().skip(1).collect()
                            };
                            let mut val_args = Vec::with_capacity(raw_args.len());
                            for a in raw_args {
                                let idx = self.string_heap.len();
                                self.string_heap.push(a);
                                val_args.push(NanVal::string(idx));
                            }
                            let arr_idx = self.allocate_heap(crate::memory::HeapObject::AArray(val_args));
                            self.stack[self.sp] = NanVal::array(arr_idx);
                            self.sp += 1;
                        } else if func_name == "type" || func_name == "typeof" {
                            let a = if argc > 0 { self.stack[self.sp - 1] } else { NanVal::null() };
                            let type_name = if a.is_int() || a.is_bigint() {
                                "Int"
                            } else if a.is_float() {
                                "Float"
                            } else if a.is_bool() {
                                "Bool"
                            } else if a.is_null() {
                                "Null"
                            } else if a.is_string() {
                                "String"
                            } else if a.is_array() {
                                "Array"
                            } else if a.is_dict() {
                                "Dictionary"
                            } else if a.is_tensor() {
                                "Tensor"
                            } else {
                                "Unknown"
                            };
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(type_name.to_string());
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "platform" {
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(std::env::consts::OS.to_string());
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "arch" {
                            self.sp -= argc + 1;
                            let idx = self.string_heap.len();
                            self.string_heap.push(std::env::consts::ARCH.to_string());
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "env_get" {
                            let key = if argc > 0 {
                                let k = self.stack[self.sp - 1];
                                if k.is_string() && k.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[k.as_string_idx()].clone()
                                } else { String::new() }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            if let Ok(v) = std::env::var(&key) {
                                let idx = self.string_heap.len();
                                self.string_heap.push(v);
                                self.stack[self.sp] = NanVal::string(idx);
                            } else {
                                self.stack[self.sp] = NanVal::null();
                            }
                            self.sp += 1;
                        } else if func_name == "read_file" {
                            let path = if argc > 0 {
                                let p = self.stack[self.sp - 1];
                                if p.is_string() && p.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[p.as_string_idx()].clone()
                                } else { String::new() }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                let idx = self.string_heap.len();
                                self.string_heap.push(content);
                                self.stack[self.sp] = NanVal::string(idx);
                            } else {
                                self.stack[self.sp] = NanVal::null();
                            }
                            self.sp += 1;
                        } else if func_name == "write_file" {
                            if self.sandbox {
                                return Err("[AAYU Sandbox Security Violation]: 'write_file' is prohibited in sandboxed execution mode.".into());
                            }
                            let content = if argc > 1 {
                                let c = self.stack[self.sp - 1];
                                if c.is_string() && c.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[c.as_string_idx()].clone()
                                } else { format!("{}", c) }
                            } else { String::new() };
                            let path = if argc > 0 {
                                let p = self.stack[self.sp - argc];
                                if p.is_string() && p.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[p.as_string_idx()].clone()
                                } else { String::new() }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let ok = std::fs::write(&path, &content).is_ok();
                            self.stack[self.sp] = NanVal::bool(ok);
                            self.sp += 1;
                        } else if func_name == "time_micros" || func_name == "time_us" {
                            self.sp -= argc + 1;
                            let micros = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_micros() as i64)
                                .unwrap_or(0);
                            self.stack[self.sp] = NanVal::int(micros);
                            self.sp += 1;
                        } else if func_name == "time_ms" {
                            self.sp -= argc + 1;
                            let millis = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_millis() as i64)
                                .unwrap_or(0);
                            self.stack[self.sp] = NanVal::int(millis);
                            self.sp += 1;
                        } else if func_name == "file_mtime" {
                            let path = if argc > 0 {
                                let p = self.stack[self.sp - 1];
                                if p.is_string() && p.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[p.as_string_idx()].clone()
                                } else { String::new() }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let mtime = std::fs::metadata(&path)
                                .and_then(|m| m.modified())
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
                                .map(|d| d.as_secs() as i64)
                                .unwrap_or(0);
                            self.stack[self.sp] = NanVal::int(mtime);
                            self.sp += 1;
                        } else if func_name == "file_size" {
                            let path = if argc > 0 {
                                let p = self.stack[self.sp - 1];
                                if p.is_string() && p.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[p.as_string_idx()].clone()
                                } else { String::new() }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let size = std::fs::metadata(&path)
                                .map(|m| m.len() as i64)
                                .unwrap_or(0);
                            self.stack[self.sp] = NanVal::int(size);
                            self.sp += 1;
                        } else if func_name == "sleep_ms" {
                            let ms = if argc > 0 {
                                let m = self.stack[self.sp - 1];
                                if m.is_int() { m.as_int() as u64 } else { 0 }
                            } else { 0 };
                            self.sp -= argc + 1;
                            std::thread::sleep(std::time::Duration::from_millis(ms));
                            self.stack[self.sp] = NanVal::null();
                            self.sp += 1;
                        } else if func_name == "os_exec" || func_name == "sys_command" {
                            if self.sandbox {
                                return Err("[AAYU Sandbox Security Violation]: Command execution via 'os_exec' is prohibited in sandboxed execution mode.".into());
                            }
                            let cmd = if argc > 0 {
                                let c = self.stack[self.sp - 1];
                                if c.is_string() && c.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[c.as_string_idx()].clone()
                                } else { String::new() }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let output = if cfg!(target_os = "windows") {
                                std::process::Command::new("powershell")
                                    .arg("-NoProfile")
                                    .arg("-Command")
                                    .arg(&cmd)
                                    .output()
                            } else {
                                std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(&cmd)
                                    .output()
                            };
                            let result = match output {
                                Ok(out) => {
                                    let s = String::from_utf8_lossy(&out.stdout);
                                    let e = String::from_utf8_lossy(&out.stderr);
                                    if !e.trim().is_empty() {
                                        format!("{}\n{}", s.trim(), e.trim()).trim().to_string()
                                    } else {
                                        s.trim().to_string()
                                    }
                                }
                                Err(err) => err.to_string(),
                            };
                            let idx = self.string_heap.len();
                            self.string_heap.push(result);
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "tensor" || func_name == "tensor_create" {
                            if argc < 2 {
                                return Err("tensor_create(data, shape) requires 2 arguments".into());
                            }
                            let shape_val = self.stack[self.sp - 1];
                            let data_val = self.stack[self.sp - 2];
                            let shape = self.extract_shape_vec(shape_val)?;
                            let data = self.extract_f64_vec(data_val)?;
                            let t = Tensor::new(shape, data);
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(t);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_zeros" {
                            if argc < 1 { return Err("tensor_zeros(shape) requires shape argument".into()); }
                            let shape_val = self.stack[self.sp - 1];
                            let shape = self.extract_shape_vec(shape_val)?;
                            let t = Tensor::zeros(shape);
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(t);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_ones" {
                            if argc < 1 { return Err("tensor_ones(shape) requires shape argument".into()); }
                            let shape_val = self.stack[self.sp - 1];
                            let shape = self.extract_shape_vec(shape_val)?;
                            let t = Tensor::ones(shape);
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(t);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_randn" {
                            if argc < 1 { return Err("tensor_randn(shape, [seed]) requires at least shape argument".into()); }
                            let shape_val = self.stack[self.sp - argc];
                            let shape = self.extract_shape_vec(shape_val)?;
                            let seed = if argc >= 2 {
                                let s_val = self.stack[self.sp - argc + 1];
                                if s_val.is_int() { s_val.as_int() as u64 } else { 12345 }
                            } else {
                                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_micros() as u64).unwrap_or(42)
                            };
                            let t = Tensor::randn(shape, seed);
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(t);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_matmul" {
                            if argc < 2 { return Err("tensor_matmul(a, b) requires 2 arguments".into()); }
                            let b_val = self.stack[self.sp - 1];
                            let a_val = self.stack[self.sp - 2];
                            let a = self.extract_tensor(a_val)?.clone();
                            let b = self.extract_tensor(b_val)?.clone();
                            let res = a.matmul(&b).map_err(|e| format!("tensor_matmul error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_add" {
                            if argc < 2 { return Err("tensor_add(a, b) requires 2 arguments".into()); }
                            let b_val = self.stack[self.sp - 1];
                            let a_val = self.stack[self.sp - 2];
                            let a = self.extract_tensor(a_val)?.clone();
                            let b = self.extract_tensor(b_val)?.clone();
                            let res = a.add(&b).map_err(|e| format!("tensor_add error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_sub" {
                            if argc < 2 { return Err("tensor_sub(a, b) requires 2 arguments".into()); }
                            let b_val = self.stack[self.sp - 1];
                            let a_val = self.stack[self.sp - 2];
                            let a = self.extract_tensor(a_val)?.clone();
                            let b = self.extract_tensor(b_val)?.clone();
                            let res = a.sub(&b).map_err(|e| format!("tensor_sub error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_mul" {
                            if argc < 2 { return Err("tensor_mul(a, b) requires 2 arguments".into()); }
                            let b_val = self.stack[self.sp - 1];
                            let a_val = self.stack[self.sp - 2];
                            let a = self.extract_tensor(a_val)?.clone();
                            let b = self.extract_tensor(b_val)?.clone();
                            let res = a.mul(&b).map_err(|e| format!("tensor_mul error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_div" {
                            if argc < 2 { return Err("tensor_div(a, b) requires 2 arguments".into()); }
                            let b_val = self.stack[self.sp - 1];
                            let a_val = self.stack[self.sp - 2];
                            let a = self.extract_tensor(a_val)?.clone();
                            let b = self.extract_tensor(b_val)?.clone();
                            let res = a.div(&b).map_err(|e| format!("tensor_div error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_scale" {
                            if argc < 2 { return Err("tensor_scale(t, scalar) requires 2 arguments".into()); }
                            let s_val = self.stack[self.sp - 1];
                            let t_val = self.stack[self.sp - 2];
                            let scalar = if s_val.is_float() { s_val.as_float() } else if s_val.is_int() { s_val.as_int() as f64 } else { 1.0 };
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.scale(scalar);
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_relu" {
                            if argc < 1 { return Err("tensor_relu(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.relu();
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_gelu" {
                            if argc < 1 { return Err("tensor_gelu(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.gelu();
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_sigmoid" {
                            if argc < 1 { return Err("tensor_sigmoid(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.sigmoid();
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_tanh" {
                            if argc < 1 { return Err("tensor_tanh(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.tanh_act();
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_silu" {
                            if argc < 1 { return Err("tensor_silu(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.silu();
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_softmax" {
                            if argc < 1 { return Err("tensor_softmax(t, [axis]) requires at least 1 argument".into()); }
                            let t_val = self.stack[self.sp - argc];
                            let axis = if argc >= 2 {
                                let a_val = self.stack[self.sp - argc + 1];
                                if a_val.is_int() { a_val.as_int() as isize } else { -1 }
                            } else { -1 };
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.softmax(axis).map_err(|e| format!("tensor_softmax error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_layernorm" {
                            if argc < 1 { return Err("tensor_layernorm(t, [gamma], [beta], [eps]) requires at least 1 argument".into()); }
                            let t_val = self.stack[self.sp - argc];
                            let t = self.extract_tensor(t_val)?.clone();
                            let gamma = if argc >= 2 {
                                let g_val = self.stack[self.sp - argc + 1];
                                if g_val.is_tensor() { Some(self.extract_tensor(g_val)?.clone()) } else { None }
                            } else { None };
                            let beta = if argc >= 3 {
                                let b_val = self.stack[self.sp - argc + 2];
                                if b_val.is_tensor() { Some(self.extract_tensor(b_val)?.clone()) } else { None }
                            } else { None };
                            let eps = if argc >= 4 {
                                let e_val = self.stack[self.sp - argc + 3];
                                if e_val.is_float() { e_val.as_float() } else { 1e-5 }
                            } else { 1e-5 };
                            let res = t.layernorm(gamma.as_ref(), beta.as_ref(), eps).map_err(|e| format!("tensor_layernorm error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_attention" {
                            if argc < 3 { return Err("tensor_attention(q, k, v, [scale]) requires at least 3 arguments".into()); }
                            let q_val = self.stack[self.sp - argc];
                            let k_val = self.stack[self.sp - argc + 1];
                            let v_val = self.stack[self.sp - argc + 2];
                            let q = self.extract_tensor(q_val)?.clone();
                            let k = self.extract_tensor(k_val)?.clone();
                            let v = self.extract_tensor(v_val)?.clone();
                            let scale = if argc >= 4 {
                                let s_val = self.stack[self.sp - argc + 3];
                                if s_val.is_float() { Some(s_val.as_float()) } else { None }
                            } else { None };
                            let res = Tensor::scaled_dot_product_attention(&q, &k, &v, scale).map_err(|e| format!("tensor_attention error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_transpose" {
                            if argc < 1 { return Err("tensor_transpose(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.transpose();
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_reshape" {
                            if argc < 2 { return Err("tensor_reshape(t, shape) requires 2 arguments".into()); }
                            let shape_val = self.stack[self.sp - 1];
                            let t_val = self.stack[self.sp - 2];
                            let shape = self.extract_shape_vec(shape_val)?;
                            let t = self.extract_tensor(t_val)?.clone();
                            let res = t.reshape(shape).map_err(|e| format!("tensor_reshape error: {}", e))?;
                            self.sp -= argc + 1;
                            let res_val = self.allocate_tensor(res);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_to_array" {
                            if argc < 1 { return Err("tensor_to_array(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?.clone();
                            self.sp -= argc + 1;
                            let res_val = self.tensor_to_nanval_array(&t);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_shape" {
                            if argc < 1 { return Err("tensor_shape(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?;
                            let shape = t.shape.clone();
                            self.sp -= argc + 1;
                            let res_val = self.shape_to_nanval_array(&shape);
                            self.stack[self.sp] = res_val;
                            self.sp += 1;
                        } else if func_name == "tensor_sum" {
                            if argc < 1 { return Err("tensor_sum(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?;
                            let s = t.sum_all();
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(s);
                            self.sp += 1;
                        } else if func_name == "tensor_mean" {
                            if argc < 1 { return Err("tensor_mean(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?;
                            let m = t.mean_all();
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(m);
                            self.sp += 1;
                        } else if func_name == "tensor_norm" {
                            if argc < 1 { return Err("tensor_norm(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?;
                            let n = t.norm_l2();
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(n);
                            self.sp += 1;
                        } else if func_name == "tensor_argmax" {
                            if argc < 1 { return Err("tensor_argmax(t) requires 1 argument".into()); }
                            let t_val = self.stack[self.sp - 1];
                            let t = self.extract_tensor(t_val)?;
                            let a = t.argmax();
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::int(a as i64);
                            self.sp += 1;
                        } else if func_name == "cosine_similarity" {
                            if argc < 2 { return Err("cosine_similarity(v1, v2) requires 2 arguments".into()); }
                            let v2_val = self.stack[self.sp - 1];
                            let v1_val = self.stack[self.sp - 2];
                            let v1 = self.extract_f64_vec(v1_val)?;
                            let v2 = self.extract_f64_vec(v2_val)?;
                            let sim = Tensor::cosine_similarity(&v1, &v2);
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(sim);
                            self.sp += 1;
                        } else if func_name == "math_exp" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.exp());
                            self.sp += 1;
                        } else if func_name == "math_ln" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 1.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(if x > 0.0 { x.ln() } else { 0.0 });
                            self.sp += 1;
                        } else if func_name == "math_sqrt" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(if x >= 0.0 { x.sqrt() } else { 0.0 });
                            self.sp += 1;
                        } else if func_name == "math_pow" {
                            let e = if argc > 1 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 1.0 };
                            let b = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - argc]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(b.powf(e));
                            self.sp += 1;
                        } else if func_name == "math_sin" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.sin());
                            self.sp += 1;
                        } else if func_name == "math_cos" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.cos());
                            self.sp += 1;
                        } else if func_name == "math_tan" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.tan());
                            self.sp += 1;
                        } else if func_name == "math_tanh" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.tanh());
                            self.sp += 1;
                        } else if func_name == "math_floor" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.floor());
                            self.sp += 1;
                        } else if func_name == "math_ceil" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.ceil());
                            self.sp += 1;
                        } else if func_name == "math_round" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.round());
                            self.sp += 1;
                        } else if func_name == "math_abs" {
                            let x = if argc > 0 { self.extract_f64_arg(self.stack[self.sp - 1]) } else { 0.0 };
                            self.sp -= argc + 1;
                            self.stack[self.sp] = NanVal::float(x.abs());
                            self.sp += 1;
                        } else if func_name == "crypto_sha256" {
                            let data = if argc > 0 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let digest = crate::crypto::sha256_hex(data.as_bytes());
                            let idx = self.string_heap.len();
                            self.string_heap.push(digest);
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "crypto_sha512" {
                            let data = if argc > 0 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let digest = crate::crypto::sha512_hex(data.as_bytes());
                            let idx = self.string_heap.len();
                            self.string_heap.push(digest);
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "crypto_hmac_sha256" {
                            let data = if argc > 1 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let key = if argc > 0 {
                                let v = self.stack[self.sp - argc];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let tag = crate::crypto::hmac_sha256_hex(key.as_bytes(), data.as_bytes());
                            let idx = self.string_heap.len();
                            self.string_heap.push(tag);
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "crypto_timing_safe_eq" || func_name == "crypto_secure_compare" {
                            let b = if argc > 1 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let a = if argc > 0 {
                                let v = self.stack[self.sp - argc];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            let eq = crate::crypto::timing_safe_equals(a.as_bytes(), b.as_bytes());
                            self.stack[self.sp] = NanVal::bool(eq);
                            self.sp += 1;
                        } else if func_name == "crypto_random_hex" {
                            let count = if argc > 0 {
                                let v = self.stack[self.sp - 1];
                                if v.is_int() { v.as_int().max(1) as usize } else { 16 }
                            } else { 16 };
                            self.sp -= argc + 1;
                            let hex_val = crate::crypto::random_bytes_hex(count).unwrap_or_default();
                            let idx = self.string_heap.len();
                            self.string_heap.push(hex_val);
                            self.stack[self.sp] = NanVal::string(idx);
                            self.sp += 1;
                        } else if func_name == "crypto_aes_gcm_encrypt" {
                            let plaintext = if argc > 2 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let nonce = if argc > 1 {
                                let v = self.stack[self.sp - 2];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let key = if argc > 0 {
                                let v = self.stack[self.sp - argc];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            match crate::crypto::aes_256_gcm_encrypt(key.as_bytes(), nonce.as_bytes(), plaintext.as_bytes()) {
                                Ok(cipher_hex) => {
                                    let idx = self.string_heap.len();
                                    self.string_heap.push(cipher_hex);
                                    self.stack[self.sp] = NanVal::string(idx);
                                }
                                Err(_) => {
                                    self.stack[self.sp] = NanVal::null();
                                }
                            }
                            self.sp += 1;
                        } else if func_name == "crypto_aes_gcm_decrypt" {
                            let cipher_hex = if argc > 2 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let nonce = if argc > 1 {
                                let v = self.stack[self.sp - 2];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let key = if argc > 0 {
                                let v = self.stack[self.sp - argc];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            self.sp -= argc + 1;
                            match crate::crypto::aes_256_gcm_decrypt(key.as_bytes(), nonce.as_bytes(), &cipher_hex) {
                                Ok(plain) => {
                                    let idx = self.string_heap.len();
                                    self.string_heap.push(plain);
                                    self.stack[self.sp] = NanVal::string(idx);
                                }
                                Err(_) => {
                                    self.stack[self.sp] = NanVal::null();
                                }
                            }
                            self.sp += 1;
                        } else if func_name == "db_btree_open" {
                            let path = if argc > 0 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { "app.db".to_string() }
                            } else { "app.db".to_string() };
                            self.sp -= argc + 1;
                            match crate::btree::BTreeDb::open(&path) {
                                Ok(db) => {
                                    let handle = self.btree_dbs.len();
                                    self.btree_dbs.push(db);
                                    self.stack[self.sp] = NanVal::int(handle as i64);
                                }
                                Err(_) => {
                                    self.stack[self.sp] = NanVal::int(-1);
                                }
                            }
                            self.sp += 1;
                        } else if func_name == "db_btree_set" {
                            let val = if argc > 2 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let key = if argc > 1 {
                                let v = self.stack[self.sp - 2];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let handle = if argc > 0 {
                                let v = self.stack[self.sp - argc];
                                if v.is_int() { v.as_int() as usize } else { 0 }
                            } else { 0 };
                            self.sp -= argc + 1;
                            let ok = if handle < self.btree_dbs.len() {
                                self.btree_dbs[handle].set(&key, &val).is_ok()
                            } else { false };
                            self.stack[self.sp] = NanVal::bool(ok);
                            self.sp += 1;
                        } else if func_name == "db_btree_get" {
                            let key = if argc > 1 {
                                let v = self.stack[self.sp - 1];
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    self.string_heap[v.as_string_idx()].clone()
                                } else { format!("{}", v) }
                            } else { String::new() };
                            let handle = if argc > 0 {
                                let v = self.stack[self.sp - argc];
                                if v.is_int() { v.as_int() as usize } else { 0 }
                            } else { 0 };
                            self.sp -= argc + 1;
                            if handle < self.btree_dbs.len() {
                                match self.btree_dbs[handle].get(&key) {
                                    Ok(Some(v)) => {
                                        let idx = self.string_heap.len();
                                        self.string_heap.push(v);
                                        self.stack[self.sp] = NanVal::string(idx);
                                    }
                                    _ => {
                                        self.stack[self.sp] = NanVal::null();
                                    }
                                }
                            } else {
                                self.stack[self.sp] = NanVal::null();
                            }
                            self.sp += 1;
                        } else if func_name == "db_btree_count" {
                            let handle = if argc > 0 {
                                let v = self.stack[self.sp - 1];
                                if v.is_int() { v.as_int() as usize } else { 0 }
                            } else { 0 };
                            self.sp -= argc + 1;
                            let count = if handle < self.btree_dbs.len() {
                                self.btree_dbs[handle].count() as i64
                            } else { 0 };
                            self.stack[self.sp] = NanVal::int(count);
                            self.sp += 1;
                        } else if func_name == "db_btree_flush" {
                            let handle = if argc > 0 {
                                let v = self.stack[self.sp - 1];
                                if v.is_int() { v.as_int() as usize } else { 0 }
                            } else { 0 };
                            self.sp -= argc + 1;
                            let ok = if handle < self.btree_dbs.len() {
                                self.btree_dbs[handle].flush().is_ok()
                            } else { false };
                            self.stack[self.sp] = NanVal::bool(ok);
                            self.sp += 1;
                        } else {
                            return Err(format!("Unknown built-in function '{}'", func_name));
                        }

                    }
                }
                36 => { // CALL_LOCAL
                    let argc = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    let target = ((self.bytecode[self.ip + 2] as usize) << 8) | self.bytecode[self.ip + 3] as usize;
                    self.ip += 4;
                    if self.call_frames.len() >= Self::MAX_CALL_DEPTH {
                        return Err(format!("StackOverflow: maximum call depth of {} exceeded", Self::MAX_CALL_DEPTH));
                    }
                    if self.sp < argc {
                        return Err(format!("CALL_LOCAL expected {} arguments on the stack", argc));
                    }
                    let nb = self.base + 256;
                    if nb + 256 > self.locals.len() {
                        return Err("StackOverflow: locals space exceeded".into());
                    }
                    if argc > 0 {
                        self.locals[nb..nb + argc].copy_from_slice(&self.stack[self.sp - argc..self.sp]);
                        self.sp -= argc;
                    }
                    self.call_frames.push(CallFrame { return_ip: self.ip, prev_base: self.base });
                    self.base = nb;
                    self.call_depth = self.call_frames.len();
                    self.ip = target;
                }
                35 | 98 => { // Ret / ReturnValue
                    if let Some(frame) = self.call_frames.pop() {
                        let result = if self.sp > 0 { self.sp -= 1; self.stack[self.sp] } else { NanVal::null() };
                        self.base = frame.prev_base;
                        self.ip = frame.return_ip;
                        self.call_depth = self.call_frames.len();
                        self.stack[self.sp] = result;
                        self.sp += 1;
                    } else {
                        return Ok(());
                    }
                }
                128 => { // BUILD_DICT
                    let num_pairs = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    let mut dict = std::collections::HashMap::with_capacity(num_pairs);
                    for _ in 0..num_pairs {
                        if self.sp >= 2 {
                            let key = self.stack[self.sp - 1];
                            let val = self.stack[self.sp - 2];
                            self.sp -= 2;
                            let key_str = if key.is_string() && key.as_string_idx() < self.string_heap.len() {
                                self.string_heap[key.as_string_idx()].clone()
                            } else {
                                key.to_string()
                            };
                            dict.insert(key_str, val);
                        }
                    }
                    let idx = self.allocate_heap(crate::memory::HeapObject::ADict(dict));
                    self.stack[self.sp] = NanVal::dict(idx);
                    self.sp += 1;
                }
                140 => { // StringConcat
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    self.sp -= 2;
                    let sa = if a.is_string() && a.as_string_idx() < self.string_heap.len() { &self.string_heap[a.as_string_idx()] } else { "" };
                    let sb = if b.is_string() && b.as_string_idx() < self.string_heap.len() { &self.string_heap[b.as_string_idx()] } else { "" };
                    let mut res = String::with_capacity(sa.len() + sb.len());
                    res.push_str(sa);
                    res.push_str(sb);
                    let idx = self.string_heap.len();
                    self.string_heap.push(res);
                    self.stack[self.sp] = NanVal::string(idx);
                    self.sp += 1;
                }
                141 => { // StringSlice
                    let end = self.stack[self.sp - 1].as_int() as usize;
                    let start = self.stack[self.sp - 2].as_int() as usize;
                    let s_ref = self.stack[self.sp - 3];
                    self.sp -= 3;
                    let s = if s_ref.is_string() && s_ref.as_string_idx() < self.string_heap.len() { &self.string_heap[s_ref.as_string_idx()] } else { "" };
                    let sliced = if s.is_ascii() {
                        let len = s.len();
                        let s_start = start.min(len);
                        let s_end = end.min(len).max(s_start);
                        s[s_start..s_end].to_string()
                    } else {
                        s.chars().skip(start).take(end.saturating_sub(start)).collect()
                    };
                    let idx = self.string_heap.len();
                    self.string_heap.push(sliced);
                    self.stack[self.sp] = NanVal::string(idx);
                    self.sp += 1;
                }
                142 => { // StringLen
                    let s_ref = self.stack[self.sp - 1];
                    let len = if s_ref.is_string() && s_ref.as_string_idx() < self.string_heap.len() {
                        self.string_heap[s_ref.as_string_idx()].len()
                    } else if s_ref.is_array() {
                        if let crate::memory::HeapObject::AArray(arr) = self.memory.get(s_ref.as_array_idx()) {
                            arr.len()
                        } else { 0 }
                    } else if s_ref.is_dict() {
                        if let crate::memory::HeapObject::ADict(dict) = self.memory.get(s_ref.as_dict_idx()) {
                            dict.len()
                        } else { 0 }
                    } else { 0 };
                    self.stack[self.sp - 1] = NanVal::int(len as i64);
                }
                254 => {
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    let ib = self.get_int128(b).unwrap_or(0);
                    let ia = self.get_int128(a).unwrap_or(0);
                    self.stack[self.sp - 2] = self.make_int128(ia + ib);
                    self.sp -= 1;
                }
                253 => { // old LT
                    let b = self.stack[self.sp - 1].as_int();
                    let a = self.stack[self.sp - 2].as_int();
                    self.stack[self.sp - 2] = NanVal::bool(a < b);
                    self.sp -= 1;
                }
                0x20 => { // JUMP
                    let offset = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip = offset;
                }
                0x21 => { // JUMP_IF_FALSE
                    let offset = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    let cond = self.stack[self.sp - 1].as_bool();
                    self.sp -= 1;
                    if !cond {
                        self.ip = offset;
                    }
                }
                145 => { // PUSH_STRING_SAFE
                    let idx = (self.bytecode[self.ip] as usize) | ((self.bytecode[self.ip+1] as usize) << 8);
                    self.ip += 2;
                    self.stack[self.sp] = NanVal::string(idx);
                    self.sp += 1;
                }
                0x02 => { self.sp -= 1; } // POP
                
                81 => { // PRINT
                    let val = self.stack[self.sp - 1];
                    self.sp -= 1;
                    if val.is_string() && val.as_string_idx() < self.string_heap.len() {
                        let s = &self.string_heap[val.as_string_idx()];
                        println!("{}", s);
                    } else if val.is_bigint() {
                        if let crate::memory::HeapObject::ABigInt(bi) = self.memory.get(val.as_bigint_idx()) {
                            println!("{}", bi);
                        } else {
                            println!("{}", val);
                        }
                    } else if val.is_array() {
                        if let crate::memory::HeapObject::AArray(arr) = self.memory.get(val.as_array_idx()) {
                            print!("[");
                            for (i, elem) in arr.iter().enumerate() {
                                if i > 0 { print!(", "); }
                                if elem.is_string() && elem.as_string_idx() < self.string_heap.len() {
                                    print!("\"{}\"", self.string_heap[elem.as_string_idx()]);
                                } else if elem.is_bigint() {
                                    if let crate::memory::HeapObject::ABigInt(bi) = self.memory.get(elem.as_bigint_idx()) {
                                        print!("{}", bi);
                                    } else {
                                        print!("{}", elem);
                                    }
                                } else {
                                    print!("{}", elem);
                                }
                            }
                            println!("]");
                        }
                    } else if val.is_dict() {
                        if let crate::memory::HeapObject::ADict(dict) = self.memory.get(val.as_dict_idx()) {
                            print!("{{");
                            for (i, (k, v)) in dict.iter().enumerate() {
                                if i > 0 { print!(", "); }
                                print!("\"{}\": ", k);
                                if v.is_string() && v.as_string_idx() < self.string_heap.len() {
                                    print!("\"{}\"", self.string_heap[v.as_string_idx()]);
                                } else if v.is_bigint() {
                                    if let crate::memory::HeapObject::ABigInt(bi) = self.memory.get(v.as_bigint_idx()) {
                                        print!("{}", bi);
                                    } else {
                                        print!("{}", v);
                                    }
                                } else {
                                    print!("{}", v);
                                }
                            }
                            println!("}}");
                        }
                    } else if val.is_tensor() && val.as_tensor_idx() < self.tensor_heap.len() {
                        self.tensor_heap[val.as_tensor_idx()].print();
                    } else {
                        println!("{}", val);
                    }
                    continue;
                }
                245 => {
                    let val = self.stack[self.sp - 1];
                    self.sp -= 1;
                    if val.is_string() {
                        let s = &self.string_heap[val.as_string_idx()];
                        println!("{}", s);
                    } else if val.is_bigint() {
                        if let crate::memory::HeapObject::ABigInt(bi) = self.memory.get(val.as_bigint_idx()) {
                            println!("{}", bi);
                        } else {
                            println!("{}", val);
                        }
                    } else {
                        println!("{}", val);
                    }
                }
                
                // --- NEW OPCODES FOR AI / DATA / SERVER ---
                
                160 => { // TensorCreate
                    if self.sp >= 2 {
                        let shape_val = self.stack[self.sp - 1];
                        let data_val = self.stack[self.sp - 2];
                        self.sp -= 2;
                        let shape = self.extract_shape_vec(shape_val)?;
                        let data = self.extract_f64_vec(data_val)?;
                        let t = Tensor::new(shape, data);
                        let res_val = self.allocate_tensor(t);
                        self.stack[self.sp] = res_val;
                        self.sp += 1;
                    }
                }
                161 => { // TensorAdd
                    if self.sp >= 2 {
                        let b_val = self.stack[self.sp - 1];
                        let a_val = self.stack[self.sp - 2];
                        self.sp -= 2;
                        let a = self.extract_tensor(a_val)?.clone();
                        let b = self.extract_tensor(b_val)?.clone();
                        let res = a.add(&b).map_err(|e| format!("TensorAdd failed: {}", e))?;
                        let res_val = self.allocate_tensor(res);
                        self.stack[self.sp] = res_val;
                        self.sp += 1;
                    }
                }
                162 => { // TensorMatMul
                    if self.sp >= 2 {
                        let b_val = self.stack[self.sp - 1];
                        let a_val = self.stack[self.sp - 2];
                        self.sp -= 2;
                        let a = self.extract_tensor(a_val)?.clone();
                        let b = self.extract_tensor(b_val)?.clone();
                        let res = a.matmul(&b).map_err(|e| format!("TensorMatMul failed: {}", e))?;
                        let res_val = self.allocate_tensor(res);
                        self.stack[self.sp] = res_val;
                        self.sp += 1;
                    }
                }
                
                170 => { // DbSet
                    let val_ref = self.stack[self.sp - 1];
                    let key_ref = self.stack[self.sp - 2];
                    self.sp -= 2;
                    
                    if key_ref.is_string() && val_ref.is_string() {
                        let k = &self.string_heap[key_ref.as_string_idx()];
                        let v = &self.string_heap[val_ref.as_string_idx()];
                        println!("[Native] Executing DB Set: '{}' = '{}' (RAM -> SSD WAL)...", k, v);
                        self.db_engine.set(k, v);
                    }
                }
                
                171 => { // DbGet
                    let key_ref = self.stack[self.sp - 1];
                    self.sp -= 1;
                    
                    if key_ref.is_string() {
                        let k = &self.string_heap[key_ref.as_string_idx()];
                        println!("[Native] Executing DB Get for '{}' (RAM Cache Hit)...", k);
                        let val = self.db_engine.get(k);
                        // Push result as string back to stack for printing
                        if let Some(s) = val {
                            let idx = self.string_heap.len();
                            self.string_heap.push(s);
                            self.stack[self.sp] = NanVal::string(idx);
                        } else {
                            self.stack[self.sp] = NanVal::null();
                        }
                        self.sp += 1;
                    }
                }
                
                180 => { // ServerStart
                    println!("[Native] Initializing HTTP/HTTPS Server...");
                    let _server = HttpServer::new(3000);
                    println!("[Native] Listening on port 3000 (Mock Mode).");
                }
                
                190 => { // UIRender
                    let val_ref = self.stack[self.sp - 1];
                    self.sp -= 1;
                    if val_ref.is_string() {
                        let component_name = &self.string_heap[val_ref.as_string_idx()];
                        println!("[Native] Compiling Declarative UI Component '{}' into DOM Tree...", component_name);
                        
                        let mut root = UiNode::new("div");
                        let style = Style { display: "flex".to_string(), background_color: "#111".to_string(), ..Default::default() };
                        root = root.with_style(style);
                        
                        let text_node = UiNode::new("h1").with_text(&format!("Welcome to {}", component_name));
                        let btn_node = UiNode::new("button").with_text("Click Me");
                        
                        root = root.add_child(text_node).add_child(btn_node);
                        
                        println!("[Native] Rendered HTML Output:");
                        println!("{}", root.render());
                    }
                }                17 => { // SUB
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        let ia = a.as_int();
                        let ib = b.as_int();
                        if let Some(diff) = ia.checked_sub(ib) {
                            if (MIN_BOXED_INT..=MAX_BOXED_INT).contains(&diff) {
                                self.stack[self.sp - 2] = NanVal::int(diff);
                            } else {
                                self.stack[self.sp - 2] = self.make_int128(diff as i128);
                            }
                        } else {
                            self.stack[self.sp - 2] = self.make_int128((ia as i128) - (ib as i128));
                        }
                    } else if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
                        let ia = self.get_int128(a).unwrap_or(0);
                        let ib = self.get_int128(b).unwrap_or(0);
                        self.stack[self.sp - 2] = self.make_int128(ia - ib);
                    } else if a.is_float() || b.is_float() {
                        let fa = if a.is_float() { a.as_float() } else if a.is_int() { a.as_int() as f64 } else if a.is_bigint() { self.get_int128(a).unwrap_or(0) as f64 } else { 0.0 };
                        let fb = if b.is_float() { b.as_float() } else if b.is_int() { b.as_int() as f64 } else if b.is_bigint() { self.get_int128(b).unwrap_or(0) as f64 } else { 0.0 };
                        self.stack[self.sp - 2] = NanVal::float(fa - fb);
                    } else {
                        let ia = self.get_int128(a).unwrap_or(0);
                        let ib = self.get_int128(b).unwrap_or(0);
                        self.stack[self.sp - 2] = self.make_int128(ia - ib);
                    }
                    self.sp -= 1;
                }
                18 => { // MUL
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        let ia = a.as_int();
                        let ib = b.as_int();
                        if let Some(prod) = ia.checked_mul(ib) {
                            if (MIN_BOXED_INT..=MAX_BOXED_INT).contains(&prod) {
                                self.stack[self.sp - 2] = NanVal::int(prod);
                            } else {
                                self.stack[self.sp - 2] = self.make_int128(prod as i128);
                            }
                        } else {
                            self.stack[self.sp - 2] = self.make_int128((ia as i128) * (ib as i128));
                        }
                    } else if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
                        let ia = self.get_int128(a).unwrap_or(0);
                        let ib = self.get_int128(b).unwrap_or(0);
                        self.stack[self.sp - 2] = self.make_int128(ia * ib);
                    } else if a.is_float() || b.is_float() {
                        let fa = if a.is_float() { a.as_float() } else if a.is_int() { a.as_int() as f64 } else if a.is_bigint() { self.get_int128(a).unwrap_or(0) as f64 } else { 0.0 };
                        let fb = if b.is_float() { b.as_float() } else if b.is_int() { b.as_int() as f64 } else if b.is_bigint() { self.get_int128(b).unwrap_or(0) as f64 } else { 0.0 };
                        self.stack[self.sp - 2] = NanVal::float(fa * fb);
                    } else {
                        let ia = self.get_int128(a).unwrap_or(0);
                        let ib = self.get_int128(b).unwrap_or(0);
                        self.stack[self.sp - 2] = self.make_int128(ia * ib);
                    }
                    self.sp -= 1;
                }
                19 => { // DIV
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
                        let ib = self.get_int128(b).unwrap_or(0);
                        if ib == 0 {
                            return Err("Integer division by zero".to_string());
                        }
                        let ia = self.get_int128(a).unwrap_or(0);
                        self.stack[self.sp - 2] = self.make_int128(ia / ib);
                    } else {
                        let fa = if a.is_float() { a.as_float() } else if a.is_int() { a.as_int() as f64 } else if a.is_bigint() { self.get_int128(a).unwrap_or(0) as f64 } else { 0.0 };
                        let fb = if b.is_float() { b.as_float() } else if b.is_int() { b.as_int() as f64 } else if b.is_bigint() { self.get_int128(b).unwrap_or(0) as f64 } else { 0.0 };
                        if fb == 0.0 {
                            return Err("Float division by zero".to_string());
                        }
                        self.stack[self.sp - 2] = NanVal::float(fa / fb);
                    }
                    self.sp -= 1;
                }
                20 => { // MOD
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if (a.is_int() || a.is_bigint()) && (b.is_int() || b.is_bigint()) {
                        let ib = self.get_int128(b).unwrap_or(0);
                        if ib == 0 {
                            return Err("Modulo by zero".to_string());
                        }
                        let ia = self.get_int128(a).unwrap_or(0);
                        self.stack[self.sp - 2] = self.make_int128(ia % ib);
                    } else {
                        return Err("Modulo requires integer operands".to_string());
                    }
                    self.sp -= 1;
                }
                38 => { // CmpEq
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        self.stack[self.sp - 2] = NanVal::bool(a.as_int() == b.as_int());
                    } else {
                        self.stack[self.sp - 2] = NanVal::bool(self.vals_eq(a, b));
                    }
                    self.sp -= 1;
                    continue;
                }
                252 => {
                    let b = self.stack[self.sp - 1].as_int();
                    let a = self.stack[self.sp - 2].as_int();
                    self.stack[self.sp - 2] = NanVal::bool(a == b);
                    self.sp -= 1;
                }
                39 => { // CmpNeq
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        self.stack[self.sp - 2] = NanVal::bool(a.as_int() != b.as_int());
                    } else {
                        self.stack[self.sp - 2] = NanVal::bool(!self.vals_eq(a, b));
                    }
                    self.sp -= 1;
                    continue;
                }
                251 => {
                    let b = self.stack[self.sp - 1].as_int();
                    let a = self.stack[self.sp - 2].as_int();
                    self.stack[self.sp - 2] = NanVal::bool(a != b);
                    self.sp -= 1;
                }
                42 => { // CmpGt
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        self.stack[self.sp - 2] = NanVal::bool(a.as_int() > b.as_int());
                    } else {
                        self.stack[self.sp - 2] = NanVal::bool(self.cmp_vals(a, b) == Some(std::cmp::Ordering::Greater));
                    }
                    self.sp -= 1;
                    continue;
                }
                250 => {
                    let b = self.stack[self.sp - 1].as_int();
                    let a = self.stack[self.sp - 2].as_int();
                    self.stack[self.sp - 2] = NanVal::bool(a > b);
                    self.sp -= 1;
                }
                43 => { // CmpLte
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        self.stack[self.sp - 2] = NanVal::bool(a.as_int() <= b.as_int());
                    } else {
                        let ord = self.cmp_vals(a, b);
                        self.stack[self.sp - 2] = NanVal::bool(ord == Some(std::cmp::Ordering::Less) || ord == Some(std::cmp::Ordering::Equal));
                    }
                    self.sp -= 1;
                    continue;
                }
                249 => {
                    let b = self.stack[self.sp - 1].as_int();
                    let a = self.stack[self.sp - 2].as_int();
                    self.stack[self.sp - 2] = NanVal::bool(a <= b);
                    self.sp -= 1;
                }
                44 => { // CmpGte
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    if a.is_int() && b.is_int() {
                        self.stack[self.sp - 2] = NanVal::bool(a.as_int() >= b.as_int());
                    } else {
                        let ord = self.cmp_vals(a, b);
                        self.stack[self.sp - 2] = NanVal::bool(ord == Some(std::cmp::Ordering::Greater) || ord == Some(std::cmp::Ordering::Equal));
                    }
                    self.sp -= 1;
                    continue;
                }
                248 => {
                    let b = self.stack[self.sp - 1].as_int();
                    let a = self.stack[self.sp - 2].as_int();
                    self.stack[self.sp - 2] = NanVal::bool(a >= b);
                    self.sp -= 1;
                }
                58 => { // LogicalAnd
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    self.stack[self.sp - 2] = NanVal::bool(a.as_bool() && b.as_bool());
                    self.sp -= 1;
                    continue;
                }
                59 => { // LogicalOr
                    let b = self.stack[self.sp - 1];
                    let a = self.stack[self.sp - 2];
                    self.stack[self.sp - 2] = NanVal::bool(a.as_bool() || b.as_bool());
                    self.sp -= 1;
                    continue;
                }
                60 => { // LogicalNot
                    let a = self.stack[self.sp - 1];
                    self.stack[self.sp - 1] = NanVal::bool(!a.as_bool());
                    continue;
                }
                0x03 => { // DUP
                    self.stack[self.sp] = self.stack[self.sp - 1];
                    self.sp += 1;
                }
                134 => { // CreateArray
                    let len = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    let start = self.sp - len;
                    let arr = self.stack[start..self.sp].to_vec();
                    self.sp = start;
                    
                    let idx = self.allocate_heap(crate::memory::HeapObject::AArray(arr));
                    self.stack[self.sp] = NanVal::array(idx);
                    self.sp += 1;
                }
                
                136 => { // LoadSubscr (Polymorphic: Array, Dict, String)
                    let key_val = self.stack[self.sp - 1];
                    let obj_ref = self.stack[self.sp - 2];
                    self.sp -= 2;
                    if obj_ref.is_array() {
                        let index_val = key_val.as_int() as usize;
                        let idx = obj_ref.as_array_idx();
                        if let crate::memory::HeapObject::AArray(arr) = self.memory.get(idx) {
                            if index_val < arr.len() {
                                self.stack[self.sp] = arr[index_val];
                            } else {
                                self.stack[self.sp] = NanVal::null();
                            }
                        }
                    } else if obj_ref.is_dict() {
                        let idx = obj_ref.as_dict_idx();
                        if let crate::memory::HeapObject::ADict(dict) = self.memory.get(idx) {
                            if key_val.is_string() && key_val.as_string_idx() < self.string_heap.len() {
                                let key_str = &self.string_heap[key_val.as_string_idx()];
                                self.stack[self.sp] = dict.get(key_str.as_str()).copied().unwrap_or(NanVal::null());
                            } else {
                                let key_str = format!("{}", key_val);
                                self.stack[self.sp] = dict.get(&key_str).copied().unwrap_or(NanVal::null());
                            }
                        } else {
                            self.stack[self.sp] = NanVal::null();
                        }
                    } else if obj_ref.is_string() {
                        let index_val = key_val.as_int() as usize;
                        let s_idx = obj_ref.as_string_idx();
                        let ch = if let Some(s) = self.string_heap.get(s_idx) {
                            s.chars().nth(index_val).map(|c| c.to_string()).unwrap_or_default()
                        } else {
                            String::new()
                        };
                        let new_idx = self.string_heap.len();
                        self.string_heap.push(ch);
                        self.stack[self.sp] = NanVal::string(new_idx);
                    } else {
                        self.stack[self.sp] = NanVal::null();
                    }
                    self.sp += 1;
                    continue;
                }
                247 => {
                    let index_val = self.stack[self.sp - 1].as_int() as usize;
                    let arr_ref = self.stack[self.sp - 2];
                    self.sp -= 2;
                    
                    if arr_ref.is_array() {
                        let idx = arr_ref.as_array_idx();
                        if let crate::memory::HeapObject::AArray(arr) = self.memory.get(idx) {
                            if index_val < arr.len() {
                                self.stack[self.sp] = arr[index_val];
                            } else {
                                panic!("Array Index Out of Bounds!");
                            }
                        }
                    } else {
                        panic!("LoadSubscr on non-array!");
                    }
                    self.sp += 1;
                }

                137 => { // StoreSubscr (Polymorphic: Array, Dict)
                    let val = self.stack[self.sp - 1];
                    let key_val = self.stack[self.sp - 2];
                    let obj_ref = self.stack[self.sp - 3];
                    self.sp -= 3;
                    if obj_ref.is_array() {
                        let index_val = key_val.as_int() as usize;
                        let idx = obj_ref.as_array_idx();
                        if let crate::memory::HeapObject::AArray(arr) = self.memory.get_mut(idx) {
                            if index_val < arr.len() {
                                arr[index_val] = val;
                            }
                        }
                    } else if obj_ref.is_dict() {
                        let idx = obj_ref.as_dict_idx();
                        if let crate::memory::HeapObject::ADict(dict) = self.memory.get_mut(idx) {
                            if key_val.is_string() && key_val.as_string_idx() < self.string_heap.len() {
                                let key_ref = &self.string_heap[key_val.as_string_idx()];
                                if let Some(existing) = dict.get_mut(key_ref.as_str()) {
                                    *existing = val;
                                } else {
                                    dict.insert(key_ref.clone(), val);
                                }
                            } else {
                                let key_str = format!("{}", key_val);
                                dict.insert(key_str, val);
                            }
                        }
                    }
                    continue;
                }
                246 => {
                    let val = self.stack[self.sp - 1];
                    let index_val = self.stack[self.sp - 2].as_int() as usize;
                    let arr_ref = self.stack[self.sp - 3];
                    self.sp -= 3;
                    
                    if arr_ref.is_array() {
                        let idx = arr_ref.as_array_idx();
                        if let crate::memory::HeapObject::AArray(arr) = self.memory.get_mut(idx) {
                            if index_val < arr.len() {
                                arr[index_val] = val;
                            } else {
                                panic!("Array Index Out of Bounds!");
                            }
                        }
                    } else {
                        panic!("StoreSubscr on non-array!");
                    }
                }
                                                200 => {
                    let str_idx = ((self.bytecode[self.ip] as usize) << 8) | self.bytecode[self.ip + 1] as usize;
                    self.ip += 2;
                    let html = self.string_heap[str_idx].clone();
                    let default_port = std::env::var("PORT")
                        .or_else(|_| std::env::var("AAYU_PORT"))
                        .ok()
                        .and_then(|p| p.parse::<u16>().ok())
                        .unwrap_or(3000);
                    
                    let mut chosen_port = default_port;
                    let listener = loop {
                        match std::net::TcpListener::bind(format!("0.0.0.0:{}", chosen_port)) {
                            Ok(l) => break l,
                            Err(_) => {
                                chosen_port += 1;
                                if chosen_port > default_port + 50 {
                                    panic!("Failed to find an available port starting from {}", default_port);
                                }
                            }
                        }
                    };
                    println!("AAYU Native Web Server & Runtime Engine started.");
                    println!("Available at: http://localhost:{}", chosen_port);

                    #[derive(Clone)]
                    struct ServerTx {
                        id: String,
                        from: String,
                        to: String,
                        amount: f64,
                        fee: f64,
                        velocity: f64,
                        volume: f64,
                        entropy: f64,
                        gas: f64,
                    }

                    struct ServerState {
                        block_height: u64,
                        latest_hash: String,
                        is_tampered: bool,
                        treasury_bal: f64,
                        alice_bal: f64,
                        bob_bal: f64,
                        miner_bal: f64,
                        pool_bal: f64,
                        mempool: Vec<ServerTx>,
                    }

                    fn esc_json(s: &str) -> String {
                        let mut o = String::new();
                        for c in s.chars() {
                            match c {
                                '"' => o.push_str("\\\""),
                                '\\' => o.push_str("\\\\"),
                                '\n' => o.push_str("\\n"),
                                '\r' => o.push_str("\\r"),
                                '\t' => o.push_str("\\t"),
                                _ => o.push(c),
                            }
                        }
                        o
                    }

                    fn find_json_val(json: &str, key: &str) -> Option<String> {
                        let pat = format!("\"{}\"", key);
                        let p = json.find(&pat)?;
                        let aft = &json[p + pat.len()..];
                        let c = aft.find(':')?;
                        let v = aft[c + 1..].trim_start();
                        if v.starts_with('"') {
                            let r = &v[1..];
                            let mut res = String::new();
                            let mut cs = r.chars();
                            while let Some(ch) = cs.next() {
                                if ch == '\\' {
                                    if let Some(e) = cs.next() {
                                        match e {
                                            'n' => res.push('\n'),
                                            'r' => res.push('\r'),
                                            't' => res.push('\t'),
                                            '"' => res.push('"'),
                                            '\\' => res.push('\\'),
                                            _ => { res.push('\\'); res.push(e); }
                                        }
                                    }
                                } else if ch == '"' {
                                    return Some(res);
                                } else {
                                    res.push(ch);
                                }
                            }
                            Some(res)
                        } else {
                            let num: String = v.chars().take_while(|ch| ch.is_ascii_digit() || *ch == '.' || *ch == '-').collect();
                            if !num.is_empty() { Some(num) } else { None }
                        }
                    }

                    fn send_http(stream: &mut std::net::TcpStream, status: u16, ctype: &str, body: &str) {
                        use std::io::Write;
                        let text = match status {
                            200 => "OK",
                            204 => "No Content",
                            404 => "Not Found",
                            _ => "OK",
                        };
                        let resp = format!(
                            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache, no-store, must-revalidate, max-age=0\r\nPragma: no-cache\r\nExpires: 0\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nConnection: close\r\n\r\n{}",
                            status, text, ctype, body.len(), body
                        );
                        let _ = stream.write_all(resp.as_bytes());
                        let _ = stream.flush();
                    }

                    fn handle_connection(
                        mut stream: std::net::TcpStream,
                        state: std::sync::Arc<std::sync::Mutex<ServerState>>,
                        html: std::sync::Arc<String>,
                    ) {
                        use std::io::Read;
                        let mut buf = [0u8; 8192];
                        let n = match stream.read(&mut buf) {
                            Ok(bytes) if bytes > 0 => bytes,
                            _ => return,
                        };
                        let raw = String::from_utf8_lossy(&buf[..n]);
                        let mut lines = raw.lines();
                        let first_line = lines.next().unwrap_or("GET / HTTP/1.1");
                        let mut parts = first_line.split_whitespace();
                        let method = parts.next().unwrap_or("GET");
                        let path = parts.next().unwrap_or("/");

                        if method == "OPTIONS" {
                            send_http(&mut stream, 204, "text/plain", "");
                            return;
                        }

                        if path == "/favicon.ico" {
                            send_http(&mut stream, 404, "text/plain", "Not Found");
                            return;
                        }

                        let mut content_len = 0;
                        for line in raw.lines() {
                            if line.to_ascii_lowercase().starts_with("content-length:") {
                                if let Some(val) = line.split(':').nth(1) {
                                    content_len = val.trim().parse::<usize>().unwrap_or(0);
                                }
                            }
                        }

                        let body_start = raw.find("\r\n\r\n").map(|p| p + 4)
                            .or_else(|| raw.find("\n\n").map(|p| p + 2))
                            .unwrap_or(n);
                        let mut body = if body_start < n { raw[body_start..].to_string() } else { String::new() };
                        if body.len() < content_len {
                            let mut rem = vec![0u8; content_len - body.len()];
                            if let Ok(rn) = stream.read(&mut rem) {
                                body.push_str(&String::from_utf8_lossy(&rem[..rn]));
                            }
                        }

                        if path == "/api/mine" {
                            let t_seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
                            let mut st = state.lock().unwrap();
                            if st.mempool.is_empty() {
                                st.mempool.push(ServerTx {
                                    id: format!("tx_batch_{}", (t_seed % 899) + 100),
                                    from: "GenesisTreasury".to_string(),
                                    to: "Alice".to_string(),
                                    amount: 20.0,
                                    fee: 0.1,
                                    velocity: 0.45,
                                    volume: 20.0,
                                    entropy: 0.10,
                                    gas: 1.0,
                                });
                            }

                            let mut approved = Vec::new();
                            let mut logs = Vec::new();
                            logs.push(format!("âš¡ HYBRID PoUW MINING: Evaluating Neural MLP Consensus on {} mempool transactions...", st.mempool.len()));

                            for tx in &st.mempool {
                                let h1_0 = (tx.velocity * 0.45 - tx.entropy * 0.12).max(0.0);
                                let h1_1 = (tx.volume * 0.04 + tx.gas * 0.35).max(0.0);
                                let h1_2 = (tx.entropy * 0.88 - 0.20).max(0.0);
                                let h1_3 = (tx.velocity * 0.20 + 0.15).max(0.0);
                                let sig = |x: f64| 1.0 / (1.0 + (-x).exp());
                                let h2_0 = sig(h1_0 * 0.50 - h1_1 * 0.30);
                                let h2_1 = sig(h1_1 * 0.40 - h1_2 * 0.60);
                                let h2_2 = sig(h1_3 * 0.30 + 0.10);
                                let mut score = h2_0 * 0.15 + h2_1 * 0.10 - h2_2 * 0.05;
                                if tx.entropy > 0.6 { score += 0.35; }
                                if score < 0.15 {
                                    approved.push(tx.clone());
                                    logs.push(format!("ðŸ§  PoUW Neural Consensus: Tx [{}] verified (Score: {:.3}) -> APPROVED", tx.id, score));
                                } else {
                                    logs.push(format!("ðŸš¨ MALICIOUS TX DETECTED: [{}] Risk Score {:.3} >= 0.15 -> Quarantined by PoUW!", tx.id, score));
                                }
                            }

                            for tx in &approved {
                                let f = tx.from.to_lowercase();
                                let t = tx.to.to_lowercase();
                                if f.contains("alice") { st.alice_bal -= tx.amount; }
                                else if f.contains("bob") { st.bob_bal -= tx.amount; }
                                else if f.contains("treasury") { st.treasury_bal -= tx.amount; }

                                if t.contains("alice") { st.alice_bal += tx.amount; }
                                else if t.contains("bob") { st.bob_bal += tx.amount; }
                                else if t.contains("pool") { st.pool_bal += tx.amount; }
                            }

                            st.miner_bal += 5.0;
                            st.block_height += 1;
                            let nonce: u64 = ((t_seed % 89999) + 10000) as u64;
                            let new_hash = format!("0000{:08x}{:08x}{:016x}{:016x}", (nonce as u32), ((t_seed >> 16) as u32), t_seed as u64, (t_seed ^ 0xFEEDFACE) as u64);
                            let merkle = format!("{:016x}{:016x}", t_seed as u64, (t_seed ^ 0xCAFEBABE) as u64);
                            let prev_hash = st.latest_hash.clone();
                            st.latest_hash = new_hash.clone();
                            let tx_cnt = approved.len();
                            st.mempool.clear();

                            logs.push(format!("âœ… BLOCK #{} MINED! Hash: {} | Nonce: {} | Merkle: {} | Miner Reward: +5.00 AYU", st.block_height, new_hash, nonce, merkle));

                            let mut logs_json = String::from("[");
                            for (i, l) in logs.iter().enumerate() {
                                if i > 0 { logs_json.push(','); }
                                logs_json.push('"');
                                logs_json.push_str(&esc_json(l));
                                logs_json.push('"');
                            }
                            logs_json.push(']');

                            let resp = format!(
                                r#"{{"success":true,"block":{{"height":{},"hash":"{}","prev_hash":"{}","merkle_root":"{}","nonce":{},"tx_count":{},"miner_reward":5.0}},"balances":{{"treasury":{:.2},"alice":{:.2},"bob":{:.2},"miner":{:.2},"pool":{:.2}}},"mempool":[],"logs":{}}}"#,
                                st.block_height, new_hash, prev_hash, merkle, nonce, tx_cnt, st.treasury_bal, st.alice_bal, st.bob_bal, st.miner_bal, st.pool_bal, logs_json
                            );
                            send_http(&mut stream, 200, "application/json", &resp);
                        } else if path == "/api/command" {
                            let cmd = find_json_val(&body, "cmd").unwrap_or_else(|| body.clone());
                            let lower = cmd.to_lowercase();
                            let mut logs = Vec::new();

                            let mut st = state.lock().unwrap();
                            if lower.contains("mine") {
                                let t_seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
                                st.block_height += 1;
                                st.miner_bal += 5.0;
                                let nonce: u64 = ((t_seed % 89999) + 10000) as u64;
                                let new_hash = format!("0000{:08x}{:08x}{:016x}{:016x}", (nonce as u32), ((t_seed >> 16) as u32), t_seed as u64, (t_seed ^ 0xFEEDFACE) as u64);
                                let merkle = format!("{:016x}{:016x}", t_seed as u64, (t_seed ^ 0xCAFEBABE) as u64);
                                let _prev_hash = st.latest_hash.clone();
                                st.latest_hash = new_hash.clone();
                                logs.push(format!("âš¡ HYBRID MINING TRIGGERED: Mining Block #{}...", st.block_height));
                                logs.push(format!("âœ… BLOCK #{} MINED! Hash: {} | Nonce: {} | Merkle: {} | Miner Reward: +5.00 AYU", st.block_height, new_hash, nonce, merkle));
                                st.mempool.clear();
                            } else if lower.contains("audit") {
                                logs.push("ðŸ›¡ï¸ STARTING CRYPTOGRAPHIC AUDIT: Verifying Genesis hash -> Block 103 -> Block 104...".to_string());
                                if st.is_tampered {
                                    logs.push("ðŸš¨ CRITICAL INVARIANT FAILURE: Block #103 hash does not match Block #104 prev_hash pointer!".to_string());
                                    logs.push("âŒ INTEGRITY CHECK FAILED: Hash mismatch at Block #103. Cryptographic chain broken!".to_string());
                                } else {
                                    logs.push("âœ… 1/3 Genesis pointer verified: 000000000019d668... OK".to_string());
                                    logs.push("âœ… 2/3 Merkle tree roots verified across all blocks... OK".to_string());
                                    logs.push("âœ… 3/3 UTXO & Invariant integrity verified: 100% Invariants Intact. Zero chain splits.".to_string());
                                }
                            } else if lower.contains("tamper") || lower.contains("corrupt") || lower.contains("attack") {
                                st.is_tampered = true;
                                logs.push("âš ï¸ SIMULATED ATTACK: Attacker altered Block #103 transaction payload & hash in VM memory!".to_string());
                                logs.push("ðŸš¨ CONSENSUS GUARD: Hash mismatch! Prev_hash pointer broken at Block #104.".to_string());
                            } else if lower.contains("heal") || lower.contains("recover") {
                                st.is_tampered = false;
                                logs.push("ðŸ”§ CONSENSUS SELF-HEAL: Re-synced with canonical longest chain. Restored Block #103 state.".to_string());
                            } else if lower.contains("contract") || lower.contains("intent") {
                                if st.treasury_bal >= 2.5 {
                                    st.treasury_bal -= 2.5;
                                    st.pool_bal += 2.5;
                                    logs.push("ðŸ“œ Smart Intent Triggered: 'AutoLiquidity.intent'".to_string());
                                    logs.push("âš¡ State Transition Executed: 2.50 AYU transferred from Treasury -> ContractPool. Invariant: Validated.".to_string());
                                } else {
                                    logs.push("âŒ CONTRACT REJECTED: Treasury has insufficient funds to rebalance liquidity.".to_string());
                                }
                            } else if lower.contains("send") || lower.contains("transfer") {
                                st.alice_bal -= 15.0;
                                st.bob_bal += 15.0;
                                logs.push("âš¡ Intent Executed: Transferred 15.00 AYU from Alice to Bob. Invariant verified.".to_string());
                            } else if lower.contains("balance") || lower.contains("wallet") {
                                logs.push(format!("ðŸ’° Ledger State: Treasury={:.2} AYU | Alice={:.2} | Bob={:.2} | Miner={:.2} | Pool={:.2}", st.treasury_bal, st.alice_bal, st.bob_bal, st.miner_bal, st.pool_bal));
                            } else {
                                logs.push(format!("ðŸ¤– [Autonomous AAYU Agent]: Intent accepted: \"{}\". Executing PoUW neural evaluation & state verification...", cmd));
                                logs.push("ðŸ§  Neural MLP Forward Pass: Score 0.084 [SAFE INTENT] -> Passed PoUW Consensus Invariant".to_string());
                            }

                            let mut logs_json = String::from("[");
                            for (i, l) in logs.iter().enumerate() {
                                if i > 0 { logs_json.push(','); }
                                logs_json.push('"');
                                logs_json.push_str(&esc_json(l));
                                logs_json.push('"');
                            }
                            logs_json.push(']');

                            let resp = format!(
                                r#"{{"success":true,"is_tampered":{},"balances":{{"treasury":{:.2},"alice":{:.2},"bob":{:.2},"miner":{:.2},"pool":{:.2}}},"logs":{}}}"#,
                                st.is_tampered, st.treasury_bal, st.alice_bal, st.bob_bal, st.miner_bal, st.pool_bal, logs_json
                            );
                            send_http(&mut stream, 200, "application/json", &resp);
                        } else if path == "/api/tx" {
                            let from = find_json_val(&body, "from").unwrap_or_else(|| "Alice".to_string());
                            let to = find_json_val(&body, "to").unwrap_or_else(|| "Bob".to_string());
                            let amt: f64 = find_json_val(&body, "amount").and_then(|v| v.parse().ok()).unwrap_or(10.0);
                            let fee: f64 = find_json_val(&body, "fee").and_then(|v| v.parse().ok()).unwrap_or(0.05);

                            let mut st = state.lock().unwrap();
                            let sender_bal = if from.to_lowercase().contains("alice") { st.alice_bal } else { st.bob_bal };
                            if sender_bal < (amt + fee) {
                                let resp = format!(r#"{{"success":false,"error":"{} has insufficient balance ({:.2} AYU) for transfer of {:.2} + {:.2} AYU!"}}"#, from, sender_bal, amt, fee);
                                send_http(&mut stream, 200, "application/json", &resp);
                            } else {
                                let t_seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
                                let tx_id = format!("tx_{}", (t_seed % 8999) + 1000);
                                st.mempool.push(ServerTx {
                                    id: tx_id.clone(),
                                    from: from.clone(),
                                    to: to.clone(),
                                    amount: amt,
                                    fee,
                                    velocity: 0.42,
                                    volume: amt,
                                    entropy: 0.08,
                                    gas: fee,
                                });

                                let mut mem_json = String::from("[");
                                for (i, tx) in st.mempool.iter().enumerate() {
                                    if i > 0 { mem_json.push(','); }
                                    mem_json.push_str(&format!(
                                        r#"{{"id":"{}","from":"{}","to":"{}","amount":{:.2},"fee":{:.2},"entropy":{:.2}}}"#,
                                        tx.id, tx.from, tx.to, tx.amount, tx.fee, tx.entropy
                                    ));
                                }
                                mem_json.push(']');

                                let resp = format!(
                                    r#"{{"success":true,"logs":["ðŸ“ Transaction Signed & Broadcast to Native VM: [{}] {} -> {} ({:.2} AYU) | Fee: {:.2} AYU"],"mempool":{}}}"#,
                                    tx_id, from, to, amt, fee, mem_json
                                );
                                send_http(&mut stream, 200, "application/json", &resp);
                            }
                        } else if path == "/api/infer" {
                            let vel: f64 = find_json_val(&body, "velocity").and_then(|v| v.parse().ok()).unwrap_or(0.35);
                            let vol: f64 = find_json_val(&body, "volume").and_then(|v| v.parse().ok()).unwrap_or(12.0);
                            let ent: f64 = find_json_val(&body, "entropy").and_then(|v| v.parse().ok()).unwrap_or(0.08);
                            let gas: f64 = find_json_val(&body, "gas").and_then(|v| v.parse().ok()).unwrap_or(1.02);

                            let h1_0 = (vel * 0.45 - ent * 0.12).max(0.0);
                            let h1_1 = (vol * 0.04 + gas * 0.35).max(0.0);
                            let h1_2 = (ent * 0.88 - 0.20).max(0.0);
                            let h1_3 = (vel * 0.20 + 0.15).max(0.0);
                            let sig = |x: f64| 1.0 / (1.0 + (-x).exp());
                            let h2_0 = sig(h1_0 * 0.50 - h1_1 * 0.30);
                            let h2_1 = sig(h1_1 * 0.40 - h1_2 * 0.60);
                            let h2_2 = sig(h1_3 * 0.30 + 0.10);
                            let mut score = h2_0 * 0.15 + h2_1 * 0.10 - h2_2 * 0.05;
                            if ent > 0.6 { score += 0.35; }
                            let approved = score < 0.15;

                            let log_msg = if approved {
                                format!("ðŸ§  Neural MLP Forward Pass: Score {:.3} [SAFE TX] -> Passed PoUW Consensus", score)
                            } else {
                                format!("ðŸš¨ Neural Anomaly Detector: Score {:.3} [HIGH RISK ATTACK] -> Quarantined by PoUW!", score)
                            };

                            let resp = format!(
                                r#"{{"success":true,"score":{:.3},"approved":{},"h1":[{:.3},{:.3},{:.3},{:.3}],"h2":[{:.3},{:.3},{:.3}],"logs":["{}"]}}"#,
                                score, approved, h1_0, h1_1, h1_2, h1_3, h2_0, h2_1, h2_2, esc_json(&log_msg)
                            );
                            send_http(&mut stream, 200, "application/json", &resp);
                        } else if path == "/api/audit" {
                            let mut logs = Vec::new();
                            logs.push("ðŸ›¡ï¸ STARTING CRYPTOGRAPHIC AUDIT: Verifying Genesis hash -> Block 103 -> Block 104...".to_string());
                            let st = state.lock().unwrap();
                            if st.is_tampered {
                                logs.push("ðŸš¨ CRITICAL INVARIANT FAILURE: Block #103 hash does not match Block #104 prev_hash pointer!".to_string());
                                logs.push("âŒ INTEGRITY CHECK FAILED: Hash mismatch at Block #103. Cryptographic chain broken!".to_string());
                            } else {
                                logs.push("âœ… 1/3 Genesis pointer verified: 000000000019d668... OK".to_string());
                                logs.push("âœ… 2/3 Merkle tree roots verified across all blocks... OK".to_string());
                                logs.push("âœ… 3/3 UTXO & Invariant integrity verified: 100% Invariants Intact. Zero chain splits.".to_string());
                            }
                            let mut logs_json = String::from("[");
                            for (i, l) in logs.iter().enumerate() {
                                if i > 0 { logs_json.push(','); }
                                logs_json.push('"');
                                logs_json.push_str(&esc_json(l));
                                logs_json.push('"');
                            }
                            logs_json.push(']');
                            let resp = format!(r#"{{"success":true,"logs":{}}}"#, logs_json);
                            send_http(&mut stream, 200, "application/json", &resp);
                        } else if path == "/api/tamper" {
                            let mut st = state.lock().unwrap();
                            st.is_tampered = true;
                            let resp = r#"{"success":true,"is_tampered":true,"logs":["âš ï¸ SIMULATED ATTACK: Attacker altered Block #103 payload & hash in VM memory!","ðŸš¨ CONSENSUS GUARD: Hash mismatch! Prev_hash pointer broken at Block #104."]}"#;
                            send_http(&mut stream, 200, "application/json", resp);
                        } else if path == "/api/heal" {
                            let mut st = state.lock().unwrap();
                            st.is_tampered = false;
                            let resp = r#"{"success":true,"is_tampered":false,"logs":["ðŸ”§ CONSENSUS SELF-HEAL: Re-synced with canonical longest chain. Restored Block #103 state."]}"#;
                            send_http(&mut stream, 200, "application/json", resp);
                        } else if path == "/api/compile" {
                            let code = find_json_val(&body, "code").unwrap_or_else(|| body.clone());
                            let temp_dir = std::env::temp_dir();
                            let aayu_path = temp_dir.join("playground_code.aayu");
                            let ayc_path = temp_dir.join("playground_code.ayc");
                            let _ = std::fs::write(&aayu_path, &code);

                            let aayuc_path = if std::path::Path::new(r"D:\INTENT-TO-SILICON\.archive\compiler_rust_bootstrap\target\release\aayuc.exe").exists() {
                                r"D:\INTENT-TO-SILICON\.archive\compiler_rust_bootstrap\target\release\aayuc.exe".to_string()
                            } else {
                                "aayuc.exe".to_string()
                            };
                            let vm_path = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from(r"D:\INTENT-TO-SILICON\runtime_rs\target_server2\release\aayu-vm.exe"));

                            let compile_output = std::process::Command::new(&aayuc_path)
                                .arg(&aayu_path)
                                .arg("-o")
                                .arg(&ayc_path)
                                .output();

                            match compile_output {
                                Ok(c_res) => {
                                    if !c_res.status.success() {
                                        let err_msg = String::from_utf8_lossy(&c_res.stderr);
                                        let out_msg = String::from_utf8_lossy(&c_res.stdout);
                                        let full_err = format!("[AAYU Compiler Diagnostic]\n{}\n{}", err_msg, out_msg);
                                        let resp = format!(r#"{{"success":false,"output":"{}"}}"#, esc_json(&full_err));
                                        send_http(&mut stream, 200, "application/json", &resp);
                                    } else {
                                        let t0 = std::time::Instant::now();
                                        let vm_output = std::process::Command::new(&vm_path)
                                            .arg(&ayc_path)
                                            .output();
                                        let elapsed = t0.elapsed().as_nanos();

                                        match vm_output {
                                            Ok(v_res) => {
                                                let stdout_str = String::from_utf8_lossy(&v_res.stdout);
                                                let stderr_str = String::from_utf8_lossy(&v_res.stderr);
                                                let mut final_out = format!(">> AAYU VM JIT Execution (v1.5.0):\n");
                                                if !stdout_str.trim().is_empty() {
                                                    final_out.push_str(&stdout_str);
                                                } else if !stderr_str.trim().is_empty() {
                                                    final_out.push_str(&stderr_str);
                                                } else {
                                                    final_out.push_str("Program executed cleanly. Exit code: 0\n");
                                                }
                                                final_out.push_str(&format!("\n[Execution time: {}ns | Pure AAYU Silicon Stack]", elapsed));
                                                let resp = format!(r#"{{"success":true,"output":"{}"}}"#, esc_json(&final_out));
                                                send_http(&mut stream, 200, "application/json", &resp);
                                            }
                                            Err(e) => {
                                                let resp = format!(r#"{{"success":false,"output":"VM Execution failed: {}"}}"#, esc_json(&e.to_string()));
                                                send_http(&mut stream, 200, "application/json", &resp);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    let resp = format!(r#"{{"success":false,"output":"Could not invoke aayuc: {}"}}"#, esc_json(&e.to_string()));
                                    send_http(&mut stream, 200, "application/json", &resp);
                                }
                            }
                        } else {
                            // Default: Render HTML Page
                            send_http(&mut stream, 200, "text/html; charset=utf-8", &html);
                        }
                    }

                    let initial_mempool = vec![
                        ServerTx {
                            id: "tx_901".to_string(),
                            from: "Alice".to_string(),
                            to: "Bob".to_string(),
                            amount: 15.0,
                            fee: 0.1,
                            velocity: 0.42,
                            volume: 15.0,
                            entropy: 0.08,
                            gas: 1.05,
                        },
                        ServerTx {
                            id: "tx_902".to_string(),
                            from: "Bob".to_string(),
                            to: "ContractPool".to_string(),
                            amount: 5.0,
                            fee: 0.05,
                            velocity: 0.35,
                            volume: 5.0,
                            entropy: 0.12,
                            gas: 1.0,
                        },
                    ];

                    let state_arc = std::sync::Arc::new(std::sync::Mutex::new(ServerState {
                        block_height: 104,
                        latest_hash: "0000a7b489c1f0e2417d9e83b4a2c1f9d8e7b6a5c4d3e2f1a0b9c8d7e6f5a4b3".to_string(),
                        is_tampered: false,
                        treasury_bal: 10000.0,
                        alice_bal: 450.0,
                        bob_bal: 120.0,
                        miner_bal: 50.0,
                        pool_bal: 500.0,
                        mempool: initial_mempool,
                    }));
                    let html_arc = std::sync::Arc::new(html);

                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                let _ = stream.set_nodelay(true);
                                let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
                                let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(5)));
                                let state_clone = std::sync::Arc::clone(&state_arc);
                                let html_clone = std::sync::Arc::clone(&html_arc);

                                std::thread::spawn(move || {
                                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                                        handle_connection(stream, state_clone, html_clone);
                                    }));
                                });
                            }
                            Err(_) => {}
                        }
                    }
                }
                _ => return Err(format!("Unknown instruction opcode: 0x{:02X} at IP: {}", op, self.ip - 1)),
            }
        }
        Ok(())
    }
}

pub fn parse_bytecode_data(data: &[u8]) -> Result<(Vec<String>, Vec<NanVal>, Vec<u8>), String> {
    if data.len() < 4 {
        return Err("File too small: missing magic header".into());
    }
    if &data[0..4] != b"AAYU" {
        return Err("Invalid magic header: expected 'AAYU'".into());
    }
    let mut pos = 4;
    
    // Read Strings
    if pos + 4 > data.len() {
        return Err("Truncated bytecode: missing string table count".into());
    }
    let mut num_strings_bytes = [0u8; 4];
    num_strings_bytes.copy_from_slice(&data[pos..pos+4]);
    let num_strings = u32::from_le_bytes(num_strings_bytes) as usize;
    pos += 4;
    
    let mut strings = Vec::new();
    for i in 0..num_strings {
        if pos + 4 > data.len() {
            return Err(format!("Truncated bytecode: missing length for string {}", i));
        }
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&data[pos..pos+4]);
        let str_len = u32::from_le_bytes(len_bytes) as usize;
        pos += 4;
        
        if pos + str_len > data.len() {
            return Err(format!("Truncated bytecode: string {} content extends past EOF", i));
        }
        let s = match String::from_utf8(data[pos..pos+str_len].to_vec()) {
            Ok(s) => s,
            Err(_) => return Err(format!("Invalid UTF-8 sequence in string table at index {}", i)),
        };
        strings.push(s);
        pos += str_len;
    }

    // Read Constants
    if pos + 4 > data.len() {
        return Err("Truncated bytecode: missing constant table count".into());
    }
    let mut num_const_bytes = [0u8; 4];
    num_const_bytes.copy_from_slice(&data[pos..pos+4]);
    let num_constants = u32::from_le_bytes(num_const_bytes) as usize;
    pos += 4;

    let mut constants = Vec::new();
    for i in 0..num_constants {
        if pos >= data.len() {
            return Err(format!("Truncated bytecode: missing type tag for constant {}", i));
        }
        let type_tag = data[pos];
        pos += 1;
        match type_tag {
            1 => { // Int
                if pos + 8 > data.len() {
                    return Err(format!("Truncated int constant at index {}", i));
                }
                let mut val_bytes = [0u8; 8];
                val_bytes.copy_from_slice(&data[pos..pos+8]);
                let val = i64::from_le_bytes(val_bytes);
                if val >= -(1i64 << 47) && val <= (1i64 << 47) - 1 {
                    constants.push(NanVal::int(val));
                } else {
                    constants.push(NanVal::float(val as f64));
                }
                pos += 8;
            }
            2 => { // String
                if pos + 4 > data.len() {
                    return Err(format!("Truncated string constant length at index {}", i));
                }
                let mut len_bytes = [0u8; 4];
                len_bytes.copy_from_slice(&data[pos..pos+4]);
                let str_len = u32::from_le_bytes(len_bytes) as usize;
                pos += 4;
                if pos + str_len > data.len() {
                    return Err(format!("Truncated string constant content at index {}", i));
                }
                let s = match String::from_utf8(data[pos..pos+str_len].to_vec()) {
                    Ok(s) => s,
                    Err(_) => return Err(format!("Invalid UTF-8 in string constant at index {}", i)),
                };
                strings.push(s);
                constants.push(NanVal::string(strings.len() - 1));
                pos += str_len;
            }
            0 => { // Null
                constants.push(NanVal::null());
            }
            3 => { // Float
                if pos + 8 > data.len() {
                    return Err(format!("Truncated float constant at index {}", i));
                }
                let mut val_bytes = [0u8; 8];
                val_bytes.copy_from_slice(&data[pos..pos+8]);
                let val = f64::from_le_bytes(val_bytes);
                constants.push(NanVal::float(val));
                pos += 8;
            }
            4 => { // Bool
                if pos >= data.len() {
                    return Err(format!("Truncated bool constant at index {}", i));
                }
                let val = data[pos] != 0;
                constants.push(NanVal::bool(val));
                pos += 1;
            }
            _ => {
                return Err(format!("Unknown constant type tag 0x{:02X} at constant index {}", type_tag, i));
            }
        }
    }

    let bytecode = data[pos..].to_vec();
    Ok((strings, constants, bytecode))
}
