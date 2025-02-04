use std::thread::spawn;

pub fn test_thread() {
    let mut _x = 0u128;
    for i in 1..500_000_000_000 {
        _x+=i;
       // println!("main thread working!");
    }
    println!("Main thread finished a little bit of work ... let's go check on the worker threads.");
}


pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500_000_000_000 {
             x+=i;// some function we want to exicute via threading
    }
    println!("value of x is: {x}");
    };
    println!("Starting new worker thread .....");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("worker thread completed");

    loop {
        test_thread();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, let's get out of here!");
            break;
        }
    }
    

}