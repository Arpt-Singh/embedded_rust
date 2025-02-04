/*example of the thread spawn && thread scope

*/

use std::thread::spawn;     
struct Person {
    first_name: String,
}
pub fn test_thread_variables() {
    let age = 34;
    let person01 = Person{first_name:String::from("Arpit")};


    let print_age = || {
        println!("This is the child closure.");
        println!("Your age is: {age}");
        println!("Your name is: {}",&person01.first_name);
    };

    //let _result = spawn(print_age).join();
    std::thread::scope( |scope|{
        scope.spawn(print_age);
        scope.spawn(print_age);
        scope.spawn(print_age);
    });
    println!("giving control back to main thread.");
    println!("Your age is: {age}");
    println!("Your name is: {}",person01.first_name);

    println!("finished printing age: ");



}