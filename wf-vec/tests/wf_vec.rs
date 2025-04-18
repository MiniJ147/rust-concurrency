use std::thread::{self};



const NTHREADS: u32 = 10;

// sequential test
#[test]
fn faa_push(){
    const TEST_SUM_PER_THREAD: i32 = 2;
    let data = std::sync::Arc::new(wf_vec::new::<i32>());
    let mut threads = vec![];


    for _ in 0..NTHREADS {    
        let v = data.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..TEST_SUM_PER_THREAD{
                v.faa_push_back(1);
            }        
        }));
    }

    for t in threads {
        assert!(t.join().is_ok())
    }

    let lc = data.clone();
    let mut sum = 0;
    loop {
        match lc.faa_pop_back() {
            Some(i) => sum += i,
            None => break,
        }
    }

    assert_eq!(sum,TEST_SUM_PER_THREAD * NTHREADS as i32,"all values not pushed sum does not match");
}


