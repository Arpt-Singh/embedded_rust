use clap::{command, Arg};


fn main() {
    let match_result = command!().about("This application registers people with their doctor's office.")
    .arg(
        Arg::new("firstname")
        .short('f')
        .long("first-name")
        .aliases(["fname"])
        .required(true)
        .help("The person's first name")
    )
    .arg(
        Arg::new("lastname")
        .short('l')
        .long("last-name")
        .aliases(["lname","lastname"])
        .required(true)
        .help("This argument take person last name")
    )
    .arg(
        Arg::new("fluffy")
        .long("fluffy")
        .help("is the person wearing a fluffy coat or not")
    )
    .get_matches();
    

}
