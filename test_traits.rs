struct Person<PetType: Animal> {
    first_name: String,
    pet:PetType,
}


trait Animal {
    fn make_sound(&self) -> ();
}
struct Dog {}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog barked!");
    }
}

struct cat {}
impl Animal for cat {
    fn make_sound(&self) -> () {
        println!("cat meowed");
    }
}
struct Bear {}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("bear roared!");
    }
}
struct tiger {}
impl Animal for tiger {
    fn make_sound(&self) -> () {
        println!("tiger roared!");
    }
}
pub fn create_person() {
    let pet1 = Dog{};
    let pet2 = cat{};
    let pet3 = Bear{};
    let pet4 = tiger{};
    let p1 = Person{first_name:"Arpit".to_string(),pet:pet2};
    p1.pet.make_sound();
}