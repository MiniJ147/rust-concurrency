// holds our logic for our descriptors required in our wait-free

pub trait Descriptor {
    fn complete(&self);
}

pub mod pop {
    use super::Descriptor;

    pub struct Desc {}
    struct SubDesc {}

    impl Descriptor for Desc {
        fn complete(&self) {}
    }

    pub fn new() -> Desc {
        Desc{}
    }

  }

// push logic
pub mod push {
    use std::ptr;
    use std::sync::atomic::AtomicU8; 
    use crossbeam::atomic::AtomicCell;
    use super::Descriptor;

    //states
    const STATE_UNDECIDED: u8 = 0;
    const STATE_FAILED: u8 = 1;
    const STATE_PASSED: u8 = 2;

    pub struct Desc<T>{
        vec_ptr: ptr::NonNull<AtomicCell<T>>,
        value: T,
        pos: i32,
        state: AtomicU8, // enum for type detection 
    }

    //NOTE &mut might cause issues so we could get rid of NonNull
    pub fn new<T>(vec: *mut AtomicCell<T>, value: T, pos: i32) -> Desc<T> {
        Desc{
            vec_ptr: match ptr::NonNull::new(vec) {
                Some(ptr) => ptr,
                None => panic!("pointer passed in is not valid"),
            },
            value,
            pos,
            state: AtomicU8::new(STATE_UNDECIDED),
        }
    }

    impl <T>Descriptor for Desc<T> {
        fn complete(&self){}
    }
}
