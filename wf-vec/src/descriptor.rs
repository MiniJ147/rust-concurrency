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
    use std::sync::atomic::AtomicU8; 
    use crate::wf_vec;

    use super::Descriptor;

    //states
    const STATE_UNDECIDED: u8 = 0;
    const STATE_FAILED: u8 = 1;
    const STATE_PASSED: u8 = 2;

    //a note on lifetimes
    //allow us to say this object is valid while region a' is valid
    //in our case a' is the vector 
    //so as long as the vector stays in scope we can use this descriptor
    pub struct Desc<'a, T>{
        pub vec: &'a wf_vec::Vec<T>,
        pub value: T,
        pub pos: i32,
        pub state: AtomicU8, // enum for type detection 
    }

    //NOTE &mut might cause issues so we could get rid of NonNull
    pub fn new<'a, T: Copy>(vec: &'a wf_vec::Vec<T>, value: T, pos: i32) -> Desc<T> {
        Desc{
            vec,
            value,
            pos,
            state: AtomicU8::new(STATE_UNDECIDED),
        }
    }

    impl <'a, T: Copy>Descriptor for Desc<'a, T> {
        fn complete(&self){
        }
    }
}
