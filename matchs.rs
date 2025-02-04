pub fn test_match_int(){
    let myage = 24;
    match myage {
        25 => {
            println!("your age is 25.");

        }
        _ => {
            println!("your age is not 25.");
        }
    }
}