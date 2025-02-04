use std::fs;

pub fn read_somefile() {
    let file_toread = "./data/file.txt";
    let read_result = std::fs::read(file_toread);

    let convert_bytes_to_string = |mut a :String, v:&u8|{
        let new_char = char::from(*v);
        a.push(new_char);
        return a;

    };

    if read_result.is_ok() {
        println!("data: {}",read_result.ok().unwrap().iter().fold(String::from(""),convert_bytes_to_string));
    }
}

pub fn remove_dir() {
    let path = "./data";
    _ = std::fs::remove_dir_all(path);
}

pub fn create_files() {


    let path = "./data/file.txt";
    let text = "Hello, Arpit Singh!";
    _ = std::fs::write(path,text);

    let path1 = "./data/file01.txt";
    let text1 = "Hello, AS!";
    _ = std::fs::write(path1,text1);

    let path2 = "./data/file02.txt";
    let text2 = "Hello, Raja Singh!";
    _ = std::fs::write(path2,text2);

    let path3 = "./data/file03.txt";
    let text3 = "Hello, RS!";
    _ = std::fs::write(path3,text3);

    //_ = std::fs::remove_file(path3);

}
pub fn test_create_dir() {
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists(){
        println!("directory is already exist.....");
        return;
    }

    let create_dir_result = fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("Created new data directory!");
    }
    else {
        println!("some problem occured creating data directory. {:?}",create_dir_result.err());
    }
    

}