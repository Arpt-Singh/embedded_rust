pub mod helpers;
pub mod closures;
pub mod matchs;
pub mod optiontest;
pub mod mystructs;

pub mod test_traits;

pub mod myvec;

pub mod myhashmap;
pub mod myiters;

pub mod mydatetime;
pub mod mythread;

pub mod myscopedthread;


pub mod mymutex;
pub mod mympsc;

pub mod myfs;

pub mod myargs;



fn main(){


    //println!("hello from rust, arpit singh!");
    //test_func();
    //let myresult = helpers::namehelpers::get_full_name("arpit", "singh");
    //println!("{0}",myresult);

    //let new_age:u16 = helpers::privatefns::get_age(25);
    //println!("{}",new_age);
    //test_if();
    //test_loop();
    //test_for();
    //closures::test_closures();

    //matchs::test_match_int(); 
    // let result = optiontest::test_option_type();
    // println!("{0}",result.unwrap());

    // let strresult = optiontest::test_option_string();
    // println!("{0}",strresult.unwrap());

    // let charresult = optiontest::test_option_chartype();
    // println!("charater type selected is : {}",charresult.unwrap().to_string());


    //let  my_persion = mystructs::new_person();

    //println!("first_name {0},last_name {1},birth_year {2}.birth_month {3}",my_persion.first_name,my_persion.last_name,my_persion.birth_year,my_persion.birth_month)
    //mystructs::creat_vehicle();
    //test_traits::create_person();
    //myvec::test_vc_int();
    //myvec::test_vec_string();
    //myhashmap::test_hashmap_basics();
    //myiters::test_rust_iterators();
    //mydatetime::test_stdtime();
    //mydatetime::test_chrono();
    // mythread::test_thread();
    //mythread::spawn_thread();----->exm (1)
    //myscopedthread::test_thread_variables();
    //mymutex::test_mutex();
    //mympsc::test_mpsc();
    // myfs::test_create_dir();
    // myfs::create_files();
    // myfs::remove_dir();
    //myfs::read_somefile();
    //myargs::process_args();

}

#[allow(dead_code)]
fn test_for(){
    let ages = [14,18,26,35,31];
    let age_drive = 16;
    for value in ages {
        println!("current age is {0}",value);
        if value >= age_drive {
            println!("you are old enough to drive!");
        } else{
            println!("You need to wait a little bit more....");
        }
    }
}
#[allow(dead_code)]
fn test_loop(){
    let mut x = 1;
    loop  {

        println!("Hello!");
        if x >5{break;}
        x+=1;
    }
}
#[allow(dead_code)]
fn test_if(){
    let age_to_drive = 16u8;
    println!("Enter the person's age:");
    let myinput = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();

    let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing driver's license, because they are old enough");
    }
    else{
        println!("wait a bit longer, you are not ld enough for a driver's license!");
    }

    let driver_license = false;
    let drivers_license = if age>=16 {true} else {false};
}
#[allow(dead_code)]
fn test_func(){
    let x: f32 = 7.0;
    println!("{:?}",x);
}