struct person{
    first_name:String,
    last_name:String
}


pub fn test_closures(){
    let add = |x:i32,y:i32| {
        println!("x:{} y:{}",x,y);
        x+y
    };
    let result = add(3,8);

    let prin_result = || println!("the result is {}",result);
    prin_result();

    let mut p1 = person{first_name:"Arpit".to_string(),last_name:"Singh".to_string()};
    let mut change_name = |new_last_name:&str| p1.last_name=new_last_name.to_string();
    change_name("athakur");
    println!("{0}",p1.last_name);
}