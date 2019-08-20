use std::net::IpAddr;
use std::error::Error;
use std::io::{self, Read, ErrorKind};
use std::fs::{self, File};

fn main() -> Result<(), Box<dyn Error>> {
    let f: File = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    if let Ok(s) = read_username_from_file() {
        println!("{}", s);
    }
    let s = read_username_from_file()?;
    println!("{}", s);

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("hello.txt")
}
