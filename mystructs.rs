#[derive(Debug)]
#[allow(dead_code)]

enum VehicalColor {
    silver,
    blue,
    red,
    black,
    white,
    green
}

pub struct Person {
   pub first_name: String,
   pub  last_name:String,
   pub  birth_year:u16,
    pub birth_month:u8,
}

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicalColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicalColor) {
        self.color = new_color;
    }

    fn creat_vehicle() -> Vehicle   {
        let new_vehicle = Vehicle{manufacturer:"default".to_string(),model:"default".to_string(),year:2020,color:VehicalColor::blue};
        return new_vehicle;
    }
}

fn new_vehical()-> Vehicle {
    let mut v1= Vehicle {manufacturer:"Porsche".to_string(),model:"911".to_string(),year:2021,color:VehicalColor::green};
    v1.paint(VehicalColor::white);
    return v1;
}

pub fn creat_vehicle(){
    //let myvehicle = new_vehical();
    let mut myvehicle = Vehicle::creat_vehicle();
    myvehicle.paint(VehicalColor::white);
    println!("{:?}",myvehicle);

}

pub fn new_person() ->Person {
    let p1 = Person{first_name:"Arpit".to_string(),last_name:"Singh".to_string(),birth_year:2000,birth_month:7};
    return p1;
}