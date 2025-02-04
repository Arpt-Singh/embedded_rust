pub fn test_vc_int() {
    let mut my_ints:Vec<i32>= Vec::new();

    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(50);
    my_ints.push(60);
    my_ints.push(70);
    my_ints.push(80);
    
    
    println!("size of Vec: {:?}",my_ints.len());
    println!("size of Vec: {:?}",my_ints.capacity());
    println!("{:?}",my_ints);

    println!("first element in Vec is :{:?}",&(&my_ints).as_slice()[0..5]);
}

pub fn test_vec_string() {
    let first_names = vec!["a","b","c","d","e"];

    for first_name in  first_names.as_slice() {
        println!("Proccessing {} ......",first_name);
    }

    println!("{:?}",first_names);
}