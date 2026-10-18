use std::collections::HashMap;
use crate::vm::NanVal;

/// Enum representing all dynamically allocated structures in AAYU.
/// This acts similar to a C 'Union' but with Rust's strict safety tags.
/// All complex types live on the HEAP, while primitive types (Int, Float, Bool) 
/// live directly on the STACK via NanVal.
#[derive(Debug, Clone)]
pub enum HeapObject {
    AString(String),
    AArray(Vec<NanVal>),
    ADict(HashMap<String, NanVal>),
    ABigInt(i128),
    // Future: AClass, AClosure, etc.
}

/// A node in the Garbage Collector heap. Contains the actual data and a mark bit.
#[derive(Debug)]
pub struct GcNode {
    pub marked: bool,
    pub data: HeapObject,
}

/// The AAYU Memory Manager & Garbage Collector
pub struct MemoryManager {
    pub heap: Vec<Option<GcNode>>,
    pub free_list: Vec<usize>,
    pub total_allocated: usize,
    pub gc_threshold: usize,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            heap: Vec::with_capacity(4096),
            free_list: Vec::new(),
            total_allocated: 0,
            gc_threshold: 32768, // Optimal 32K object threshold (fits in CPU cache)
        }
    }

    /// Allocates a new object on the heap, reusing free slots if available.
    pub fn allocate(&mut self, obj: HeapObject) -> usize {
        let idx = if let Some(free_idx) = self.free_list.pop() {
            self.heap[free_idx] = Some(GcNode { marked: false, data: obj });
            free_idx
        } else {
            self.heap.push(Some(GcNode { marked: false, data: obj }));
            self.heap.len() - 1
        };
        
        self.total_allocated += 1;
        idx
    }

    pub fn get(&self, idx: usize) -> &HeapObject {
        &self.heap[idx].as_ref().expect("Memory Access Violation!").data
    }
    
    pub fn get_mut(&mut self, idx: usize) -> &mut HeapObject {
        &mut self.heap[idx].as_mut().expect("Memory Access Violation!").data
    }

    /// GARBAGE COLLECTOR: MARK PHASE
    /// Traces all active memory starting from the VM Roots (Stack, Locals, Globals, Constants).
    pub fn mark_roots(&mut self, roots: &[NanVal]) {
        let mut worklist: Vec<usize> = Vec::new();

        let heap_len = self.heap.len();
        // 1. Scan roots for pointers to the Heap (Array, Dict, BigInt)
        for val in roots {
            if val.is_array() {
                let idx = val.as_array_idx();
                if idx < heap_len { worklist.push(idx); }
            } else if val.is_dict() {
                let idx = val.as_dict_idx();
                if idx < heap_len { worklist.push(idx); }
            } else if val.is_bigint() {
                let idx = val.as_bigint_idx();
                if idx < heap_len { worklist.push(idx); }
            }
        }

        // 2. Trace references inside Heap objects (DFS)
        while let Some(idx) = worklist.pop() {
            if let Some(node) = &mut self.heap[idx] {
                if !node.marked {
                    node.marked = true; // Mark as alive
                    
                    let mut children = Vec::new();
                    match &node.data {
                        HeapObject::AArray(arr) => {
                            for v in arr {
                                if v.is_array() {
                                    let c_idx = v.as_array_idx();
                                    if c_idx < heap_len { children.push(c_idx); }
                                } else if v.is_dict() {
                                    let c_idx = v.as_dict_idx();
                                    if c_idx < heap_len { children.push(c_idx); }
                                } else if v.is_bigint() {
                                    let c_idx = v.as_bigint_idx();
                                    if c_idx < heap_len { children.push(c_idx); }
                                }
                            }
                        },
                        HeapObject::ADict(dict) => {
                            for v in dict.values() {
                                if v.is_array() {
                                    let c_idx = v.as_array_idx();
                                    if c_idx < heap_len { children.push(c_idx); }
                                } else if v.is_dict() {
                                    let c_idx = v.as_dict_idx();
                                    if c_idx < heap_len { children.push(c_idx); }
                                } else if v.is_bigint() {
                                    let c_idx = v.as_bigint_idx();
                                    if c_idx < heap_len { children.push(c_idx); }
                                }
                            }
                        },
                        _ => {}
                    }
                    worklist.extend(children);
                }
            }
        }
    }

    /// GARBAGE COLLECTOR: SWEEP PHASE
    /// Frees any memory that was not marked as alive.
    pub fn sweep(&mut self) -> usize {
        let mut freed_objects = 0;
        for i in 0..self.heap.len() {
            if let Some(node) = &mut self.heap[i] {
                if !node.marked {
                    // DEAD OBJECT - Free it!
                    self.heap[i] = None;
                    self.free_list.push(i);
                    self.total_allocated = self.total_allocated.saturating_sub(1);
                    freed_objects += 1;
                } else {
                    // LIVE OBJECT - Unmark for the next GC cycle
                    node.marked = false;
                }
            }
        }
        freed_objects
    }
}

