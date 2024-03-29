use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    // let username = read_username_from_file();
    // let username = read_username_from_file2();
    // let username = read_username_from_file3();

    // let username = read_username_from_file4();

    // let username = match username {
    //     Ok(name) => name,
    //     Err(e) => panic!("{:?}", e),
    // };

    let f = File::open("hello.txt")?;

    Ok(())
}
