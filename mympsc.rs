use std::sync::mpsc;
use std::time::Duration;

pub fn test_mpsc() {
    let (tx,rx) = mpsc::channel::<u8>(); 

    rx.recv_timeout(Duration::from_millis(300));

    // println!("receive status is: {}",receiver_result.is_ok());
    // println!("receive status is: {}",receiver_result.unwrap());
    // let receiver_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("receive status is: {}",receiver_result.is_ok());
    // println!("receive status is: {}",receiver_result.unwrap());
    
     let processor_code = move || {
        println!("starting processor thread!.......");
        let mut faild_count = 0u8;

        loop {
            println!("attempting to receive message from channel....");
            let receive_result = rx.recv_timeout(Duration::from_millis(300));
            if receive_result.is_ok(){
                print!("received message: {}",receive_result.unwrap());
            }
            else{
                faild_count += 1;

            }
            if faild_count > 10 {
                println!("aborting processor thread....no work available");
                break;
            }
        }
     };
   

   for x in 1..=6 {

    let send_data = tx.send(x);

    println!("send status : {}",send_data.is_ok());
    std::thread::sleep(Duration::from_millis(200));

   }

   std::thread::spawn(processor_code).join();



}