use serde::{Deserialize,Serialize};

use serde_json::{to_string_pretty,from_str};

#[derive(Serialize,Deserialize,Debug)]
//#[serde(rename_all = "camelCase")]
struct Dog {
    name:String,
    year_born:i32,
    owner:DogOwner,
}

#[derive(Serialize,Deserialize,Debug)]
struct DogOwner {
    first_name:String,
    last_name:String,
}

fn serialize_test() {
    let owner01 = DogOwner{first_name:"Arpit".to_string(),last_name:"Singh".to_string()};
    let dog01 = Dog{name:"Cheyenne".to_string(),year_born:2021,owner:owner01};
    let dog_ser = to_string_pretty(&dog01);

     
    if dog_ser.is_ok() {
        println!("{}",dog_ser.ok().unwrap());
    }
    else{
        println!("{:#?}",dog_ser.err());
    }
   
}



fn deserialize() {
    let json_string = r#"
    {
    "name":"Cheyenne",
    "year_born":2021,
    "owner": {
        "first_name":"Arpit",
        "last_name":"Singh"
        }
    }
    "#;
    let dog_deser = from_str::<Dog>(json_string);

    if dog_deser.is_ok() {
        print!("{:#?}",dog_deser.ok().unwrap());
    }
    else {
        println!("{:#?}",dog_deser.err());
    }

}
fn main() {
    deserialize();
    //serialize_test();
}




