use std::alloc::{Layout,self};
use std::ptr::{self, NonNull};
use std::sync::atomic::{self, AtomicI32, AtomicPtr, Ordering};
use std::usize;
use crossbeam::atomic::AtomicCell;

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

// [WARNING]: if size is greater than a word it will not longer be wait fre
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
//old sequential tests
//currently templating with a sequential vector then will move to concurrent 
//first I have to understand how the sequential vectors are built in rust
// use std::{mem, ptr};
// use std::alloc::{self,Layout};
// use std::ptr::NonNull;
//
// pub struct Vec<T> {
//     ptr: NonNull<T>,
//     cap: usize,
//     len: usize
// }
//
// impl<T> Vec<T> {
//     pub fn new() -> Self {
//         assert!(mem::size_of::<T>() != 0, "cannot handle empty data");
//         Vec {
//             ptr: NonNull::dangling(),
//             len: 0,
//             cap: 0,
//         }
//     }
//     
//     fn grow(&mut self) {
//         //getting our cap and new layout
//         let (new_cap, new_layout) = if self.cap == 0 {
//             (1,Layout::array::<T>(1).unwrap())
//         }else {
//             let new_cap = 2 * self.cap; // grow our vector
//             let new_layout = Layout::array::<T>(new_cap).unwrap();
//
//             (new_cap,new_layout)
//         };
//        
//         //ensure we didn't overflow our alloc 
//         assert!(new_layout.size() <= isize::MAX as usize, "Allocation overflow");
//
//         let new_ptr = if self.cap == 0 {
//             // init our array
//             unsafe { alloc::alloc(new_layout) }
//         } else {
//             // resizing our array
//             let old_layout = Layout::array::<T>(self.cap).unwrap();
//             let old_ptr = self.ptr.as_ptr() as *mut u8;
//
//             unsafe { alloc::realloc(old_ptr,old_layout,new_layout.size()) }
//         };
//
//         // check if our new_ptr alloc failed 
//         self.ptr = match NonNull::new(new_ptr as *mut T) {
//             Some(ptr) => ptr,
//             None => alloc::handle_alloc_error(new_layout),
//         };
//
//         self.cap = new_cap;
//     }
//
//     pub fn push_back(&mut self, val: T) {
//         if self.len == self.cap {
//             self.grow();
//         }
//
//         unsafe {
//             ptr::write(self.ptr.as_ptr().add(self.len), val);
//         }
//
//         self.len += 1;
//     }
//
//     pub fn read(self, idx: usize) -> T {
//         assert!(idx < self.len, "out of bounds index");
//         
//         unsafe{ptr::read(self.ptr.as_ptr().add(idx))}
//     }
// }
//
// pub fn example(){
//     print!("hello world!\n");
// }
//
//
