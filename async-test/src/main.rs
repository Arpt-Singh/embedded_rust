
#![allow(unused_variables)]
use futures::join;

fn main() {
   
   let num1 = num1();
   let num2 = num2();
   let num3 = num3();
   //println!("{:?}",num);
   let result =  smol::block_on(
    async {
        join!(num2, num1,num3)
    }

   );
   println!("final value is:{:?}", result);

}

async fn num1() -> u8{
   // println!("running function!");
    return 8;
}


async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(50));
    return 10;
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(75));
    return 12;
}
