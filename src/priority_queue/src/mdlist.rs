usae crossbeam::epoch::{self, Atomic, Owned, Shared, Guard};
use std::sync::atomic::Ordering;

pub const DIMENSION: usize = 8;

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

const ADPINV_MASK: usize = 1;
const PRGINV_MASK: usize = 2;
const INVALID_MASK: usize = 3;
const MARKED_MASK: usize = 1;
const DELETED_MASK: usize = 1;
