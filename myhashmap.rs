use std::collections::HashMap;

pub fn test_hashmap_basics() {
    let mut stock_list:HashMap<String, f32> = HashMap::<String,f32>::new();
    println!("{}",stock_list.len());
    println!("{}",stock_list.is_empty());
    stock_list.insert("NVDA".to_string(), 478.52);
    stock_list.insert("AAPL".to_string(), 300.52);
    stock_list.insert("AMSC".to_string(), 450.78);
    println!("{:#?}",stock_list);
    stock_list.remove(&("AAPL".to_string()));
    println!("{:#?}",stock_list);
    stock_list.insert("AAPL".to_string(), 350.52);
    println!("{:#?}",stock_list);

    for (ticker,value) in stock_list {
        println!("{} is trading at {}",ticker,value);
    }
}   