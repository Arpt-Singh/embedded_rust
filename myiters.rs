pub fn test_rust_iterators() {
    let fruit_list = vec!["strawberry","blueberry","mango","Apple","Orange"];
    let mut fruit_iter = fruit_list.iter();
    

    let nut_list = vec!["walnut","almonds","pecans","brazil nuts"];
    
    fruit_iter.next();
    fruit_iter.next();
    
    let item01 = fruit_iter.next();
    println!("first item in list is: {}",item01.unwrap());

    let aggregate_foods = fruit_list.iter().chain(&nut_list);
    
    let all_foods:Vec<&&str> = aggregate_foods.clone().collect();
    
    for food in aggregate_foods {
        println!("{}",food);
    }

    let mut fruit_list_strings = fruit_list.iter().map(|e| String::from(*e) );
    let new_fruits = fruit_list_strings.map(|mut e| {{e.push_str("fruit")};return e;});
    
    new_fruits.clone().for_each(|e:String|println!("{}",e));

    let last_item = new_fruits.clone().last();

    let mut step_list = new_fruits.clone().step_by(2);

    println!("step: {}",step_list.next().unwrap());
    println!("step: {}",step_list.next().unwrap());
    println!("step: {}",step_list.next().unwrap());
    println!("step: {}",step_list.next().unwrap());


    let first_name = vec!["Trevor","Shannon","James","Tasha"];
    let first_name_strings = first_name.iter().map(|e| String::from(*e));

    let last_name = vec!["Jones","Sullivan","Tanner","Redman"];
    let last_name_strings = last_name.iter().map(|e|String::from(*e));
    

    let full_name = first_name_strings.zip(last_name_strings);
    
    full_name.clone().for_each(|e|println!("{} {}",e.0,e.1));


    for (index,value) in full_name.enumerate() {
        println!("index: {0} value: {1} {2}",index,value.0,value.1);
    }
}