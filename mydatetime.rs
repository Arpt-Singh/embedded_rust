extern crate chrono;

use std::{ops::Sub, time::{Duration,Instant}};



pub fn test_stdtime() {
    let dur1 = Duration::from_secs(15);
    println!("{}",dur1.as_millis());

    let dur2 = Duration::from_millis(15500);
    let dur3 = dur1.checked_sub(dur2);
    println!("{}",dur3.unwrap_or_default().as_millis());


    let now = Instant::now();

    std::thread::sleep(Duration::from_millis(200));

    println!("{}",now.elapsed().as_millis());



}


pub fn test_chrono() {
   let utc_now = chrono::Utc::now();
    println!("{}",utc_now.format("%Y %b %d %H"));

    let locl_time = chrono::Local::now();
    println!("{}",locl_time.format("%Y %b %d %H"));
}