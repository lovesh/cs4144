extern crate rand;

//use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use common;


pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfile>", args[0]);
    } else {
        let fname = (&args[1]).clone();
        let path = Path::new(&fname);
        match File::open(&path) {
            Ok(mut msg_file) => {
                let mut msg_bytes = Vec::new();

                match msg_file.read_to_end(&mut msg_bytes) {
                    Err(why) => panic!("couldn't read {}: {}", path.display(),
                                       why.description()),
                    Ok(_) => {
                        print!("{} contains {} bytes:\n", path.display(), msg_bytes.len());
                        let share1_name = fname.to_owned() + ".share1";
                        let share2_name = fname.to_owned() + ".share2";
                        let share1_file = File::create(&Path::new(&share1_name));
                        let share2_file = File::create(&Path::new(&share2_name));
                        match (share1_file, share2_file) {
                            (Ok(share1), Ok(share2)) => {
                                split(&msg_bytes, share1, share2);
                            } ,
                            (_, _) => panic!("Error opening output files!"),
                        }
                    },
                }
             } ,
            Err(_) => panic!("Error opening message file: {}", fname)
        }
    }
}


fn split(msg_bytes: &Vec<u8>, mut share1: File, mut share2: File) {
    let len = msg_bytes.len();
    let mut random_bytes: Vec<u8> = Vec::with_capacity(len);
    // This is not cryptographically strong randomness!
    // (For entertainment purposes only.)
    for _ in 0..len {
        let random_byte = rand::random::<u8>();
        random_bytes.push(random_byte);
    }

    let encrypted_bytes = common::xor(msg_bytes, &random_bytes);

    share1.write(random_bytes.as_slice());
    share2.write(encrypted_bytes.as_slice());
}