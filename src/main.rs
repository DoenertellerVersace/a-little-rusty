use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf: Vec<u8> = vec![];
    // read file
    let mut f = match File::open("foo.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
    let mut _pos: u32 = 0;
    while buf.len() != 0 {
        println!("buf: {:?}", buf);

        match f.read(&mut buf)
        {
            Ok(n) => {
                println!("read {} bytes", n);
                _pos += n as u32;
            },
            Err(error) => {
                panic!("Problem reading the file: {:?}", error)
            }
        }
    }
}