pub fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8>= None;
    opt1 = Some(10);
    return  opt1;

}
pub  fn test_option_string() -> Option<String>{
let mut opt1:Option<String> = None;
opt1 = Some("Arpit Singh".to_string());
return opt1;
}


pub fn test_option_chartype() -> Option<CharaterType> {
    let mut  chartype:Option<CharaterType> = None;
    chartype = Some(CharaterType::Mage);
    return chartype;
}


pub enum CharaterType {
    Archer,
    Warrior,
    Mage
}

impl ToString for CharaterType{
    fn to_string(&self) -> String {
        match self {
            CharaterType::Archer => "Archer".to_string(),
            CharaterType::Mage => "Mage".to_string(),
            CharaterType::Warrior => "Warrior".to_string()


        } 
    }
}