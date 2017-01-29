/*
Exercise 2.3. Implement the joiner. It should take two file names as its inputs, and output to
standard output the result of XOR-ing the bytes in those files. The inputs files must be the
same length.
*/

extern crate rand;


use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::str;
use std::string;
use common;


pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <share1> <share2>", args[0]);
    } else {
        let share1_name = (&args[1]).clone();
        let share2_name = (&args[2]).clone();
//        let path1 = Path::new(&share1_name);
//        let path2 = Path::new(&share2_name);

        // TODO: See if its possible to match multiple
//        match (File::open(&path1), File::open(&path2)) {
//            Ok((mut share1_file, mut share2_file)) => {
//                let mut share1_bytes = Vec::new();
//                let mut share2_bytes = Vec::new();
//
//                match share1_file.read_to_end(&mut share1_bytes) {
//                    Err(why) => panic!("couldn't read {}: {}", path1.display(),
//                                       why.description()),
//                    Ok(_) => {
//                        println!("Successfully read {}", share1_name)
//                    },
//                }
//
//                match share2_file.read_to_end(&mut share2_bytes) {
//                    Err(why) => panic!("couldn't read {}: {}", path2.display(),
//                                       why.description()),
//                    Ok(_) => {
//                        println!("Successfully read {}", share2_name)
//                    },
//                }
//
//                println!("After xor, its: {}", xor(&share1_bytes, &share2_bytes))
//
//            } ,
//            Err(_) => panic!("Error opening message files: {} and {}", share1_name, share2_name)
//        }

        let share1 = file_contents(&share1_name);
        let share2 = file_contents(&share2_name);
        // TODO: Dont know why but doing `let buf = xor(&share1, &share2).as_slice()`
        // gives error `temporary value needs to live until here`
        let xor = common::xor(&share1, &share2);
        let buf = xor.as_slice();
        let s = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("After xor, its: {}", s);
    }
}


fn file_contents(name: &string::String) -> Vec<u8> {
    let path1 = Path::new(name);
    match File::open(&path1) {
        Ok(mut file) => {
            let mut bytes = Vec::new();

            match file.read_to_end(&mut bytes) {
                Err(why) => panic!("couldn't read {}: {}", path1.display(),
                                   why.description()),
                Ok(_) => {
                    println!("Successfully read {}", name);
                    bytes
                },
            }

        } ,
        Err(_) => panic!("Error opening file: {}", name)
    }
}