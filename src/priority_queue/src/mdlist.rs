use crossbeam::epoch::{self, Atomic, Owned, Shared, Guard};
use std::sync::atomic::Ordering;

pub const DIMENSION: usize = 8;

const ADPINV_MASK: usize = 1;
const PRGINV_MASK: usize = 2;
const INVALID_MASK: usize = 3;
const MARKED_MASK: usize = 1;
const DELETED_MASK: usize = 1;

pub struct Desc {
    curr: Atomic<Node>,
    pred_dim: u8,      
    dim: u8,          
}

pub struct Node {
    child: [Atomic<Node>; DIMENSION],
    key: u32,
    coord: [u8; DIMENSION],
    seq: u32,
    purged: Atomic<Node>,
    pending: Atomic<Desc>, 
}

pub struct Stack {
    head: Atomic<Node>,
    del: [Atomic<Node>; DIMENSION],
}

pub struct MDList {
    pool: Allocator,     
    head: Atomic<Node>,
    _pad: [u8; CACHE_LINE_SIZE - std::mem::size_of::<Atomic<Node>>()],
    stack: Atomic<Stack>,
    _pad1: [u8; CACHE_LINE_SIZE - std::mem::size_of::<Atomic<Stack>>()],
    purge: Atomic<Stack>,
}

impl MDList {
    pub fn set_adpinv(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) | ADPINV_MASK) as *mut Node
    }
    
    pub fn clr_adpinv(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) & !ADPINV_MASK) as *mut Node
    }
    
    pub fn is_adpinv(ptr: *mut Node) -> bool {
        ((ptr as usize) & ADPINV_MASK) != 0
    }
    
    pub fn set_prginv(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) | PRGINV_MASK) as *mut Node
    }
    
    pub fn clr_prginv(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) & !PRGINV_MASK) as *mut Node
    }
    
    pub fn is_prginv(ptr: *mut Node) -> bool {
        ((ptr as usize) & PRGINV_MASK) != 0
    }
    
    pub fn clr_invalid(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) & !INVALID_MASK) as *mut Node
    }
    
    pub fn is_invalid(ptr: *mut Node) -> bool {
        ((ptr as usize) & INVALID_MASK) != 0
    }
    
    pub fn set_marked(ptr: *mut Stack) -> *mut Stack {
        ((ptr as usize) | MARKED_MASK) as *mut Stack
    }
    
    pub fn clr_marked(ptr: *mut Stack) -> *mut Stack {
        ((ptr as usize) & !MARKED_MASK) as *mut Stack
    }
    
    pub fn is_marked(ptr: *mut Stack) -> bool {
        ((ptr as usize) & MARKED_MASK) != 0
    }
    
    pub fn set_deleted(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) | DELETED_MASK) as *mut Node
    }
    
    pub fn clr_deleted(ptr: *mut Node) -> *mut Node {
        ((ptr as usize) & !DELETED_MASK) as *mut Node
    }
    
    pub fn is_deleted(ptr: *mut Node) -> bool {
        ((ptr as usize) & DELETED_MASK) != 0
    }
}
