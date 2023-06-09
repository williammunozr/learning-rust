use std::fs;
use std::io;

pub fn master() {
    let f = read_username_from_file();

    let f = match f {
        Ok(username) => username,
        Err(e) => e.to_string(),
    };

    println!("In Error Propagation f is: {}", f);
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
