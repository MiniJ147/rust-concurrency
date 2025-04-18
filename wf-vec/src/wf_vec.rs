use std::alloc::{Layout,self};
use std::ptr::{self, NonNull};
use std::sync::atomic::{self, AtomicI32, AtomicPtr, Ordering};
use std::usize;
use crossbeam::atomic::AtomicCell;

use crate::descriptor::{self, Descriptor};

/*
Wait Free Vector:
    Based on: An Efficient Wait-Free Vector
    By: Steven Feldman, Carlos Valera-Leon, Damian Dechev

Design Overview:
    key operations
        push_back (val T)
        faa_push_back(val T)

        pop_back () T
        faa_pop_back() T
    
        read_at (idx usize) T
        write_at (idx usize, val T)

        resize ()

    we will reserve the 2 LSB in order for type detection and copied

faa_pop_back and faa_push_back:
    only one set of these functions can be called concurrently

Descriptors:
    in the paper they reserve the LSB for type detection, but I wonder if this will cause issues in rust
    
    Pop Descriptor - sub descriptor
        functions: complete()

    Push Descriptor - sub descriptor
        functions: copmlete()


A NOTE on Atomic Size:
Only can support 30 bit types wait-free. This is because we are taking the 2 LSB bits and if the type is greater than 30 bits
we will go over the word size for a 32-bit machine. If your word size is for 64-bit then you can support 62 bit types atomically.
*/

const TEST_CAP: usize = 100;

pub struct Vec<T>{
    size: AtomicI32,
    cap: AtomicI32,

    //[NOTE]: don't know if this is going to work atm but Im testing faa without resize alg
    ptr: AtomicPtr<AtomicCell<T>>,
}

pub fn new<T>() -> Vec<T> {
    assert!(std::mem::size_of::<T>() != 0, "cannot handle empty types");
        
    let layout = Layout::array::<AtomicCell<T>>(TEST_CAP).unwrap();
    let ptr_alloc = unsafe {alloc::alloc(layout)};

    Vec {
        size: AtomicI32::new(0),
        cap: AtomicI32::new(TEST_CAP as i32),
        ptr: match NonNull::new(ptr_alloc as *mut AtomicCell<T>){
            Some(ptr) =>AtomicPtr::new(ptr.as_ptr()),
            None => alloc::handle_alloc_error(layout),
        },
    }
}

impl<T: Copy> Vec<T>{   
    //[WARNING]: TEST NOT THREAD SAFE
    // not fully concurrent implementation as we need to implement logic for getSpot()
    pub fn faa_push_back(&self, val: T){
        let spot = self.size.fetch_add(1, Ordering::Relaxed);
        assert!(spot < self.cap.load(Ordering::Relaxed));
     
        let z = unsafe{&*self.ptr.load(Ordering::Relaxed).add(spot as usize)};
        z.store(val);    
    }

    pub fn push_back(){}
    pub fn pop_back(){}

    pub fn faa_pop_back(&self) -> Option<T>{
        let spot = self.size.fetch_sub(1, Ordering::Relaxed) - 1;
        if spot < 0 {
            self.size.fetch_add(1, Ordering::Relaxed);
            return None
        }

        let val = unsafe{&*self.ptr.load(Ordering::Relaxed).add(spot as usize)}; 
        return Some(val.load())
    }

    pub fn read_at(){}
    pub fn write_at(){}

    fn resize(){}
}
