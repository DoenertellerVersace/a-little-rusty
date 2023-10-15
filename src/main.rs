use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::thread;
use std::thread::JoinHandle;

fn main() -> io::Result<()> {
    let file = File::open("sgb-words.txt")?;
    let reader = BufReader::new(file);
    let mut this_chunk: Vec<String> = vec![];
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let lines = reader.lines();
    for (i, line) in lines.enumerate() {
        match i % 100 {
            0 => {
                handles.push(thread::spawn(move || {
                    for (j, word) in (&this_chunk).clone().iter().enumerate() {
                        println!("{}: {}", j, word);
                    }
                }));
                this_chunk = vec![];
            }
            _ => { this_chunk.push(line?); }
        }
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}