use std::{ops::AddAssign, sync::Mutex};
use std::thread::{spawn,scope, sleep};
use std::time::Duration;

pub fn test_mutex() {
    let mut score = Mutex::new(0u128);
    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);

    // println!("{:?}",data);
    // drop(data);

    let myfunc = || {
        println!("thread 1 is waiting for mutex lock ....");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("thread 1 is adding {i}");
            sleep(Duration::from_millis(400));

        }
    };

    let myfunc2 = || {
        loop {
            println!("thread 2 is waiting for mutex lock ....");
            let guard = score.try_lock();

            if guard.is_ok() {
                let mut data = guard.unwrap();

                for i in 1..10 {
                    data.add_assign(i);
                    println!("thread 2 is adding {i}");
                }
                break; 
            }
            sleep(Duration::from_millis(300));
        }
    };

    _= scope(|s| {
        s.spawn(myfunc2).join(); 
        s.spawn(myfunc).join(); // thread one is working on mutex
       
        
    });

    println!("{:?}",score.lock().unwrap()); // main thread also working thread after operation





}