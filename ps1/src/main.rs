//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 1+
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::fs;
use std::fs::File;
use std::path::Path;


const DEBUG: bool = false;

fn main() {

    let addr = "127.0.0.1:4414";

    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on [{}] ...", addr);

    static mut VISITOR_COUNT: i32 = 0;

    let root_path = "/";
    let favicon_path = "/favicon.ico";
    let forbidden = "HTTP/1.1 403 Forbidden\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><head></head><body>Forbidden</body></html>";
    let ok = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";

    for stream in listener.incoming() {
        match stream {
            Err(_) => (),
            Ok(mut stream) => {
                // Spawn a thread to handle the connection
                thread::spawn(move|| {
                    match stream.peer_addr() {
                        Err(_) => (),
                        Ok(pn) => println!("Received connection from: [{}]", pn),
                    }

                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();
                    let body = match str::from_utf8(&buf) {
                        Err(error) => {
                            println!("Received request error:\n{}", error);
                            None
                        },
                        Ok(body) => {
                            println!("Received request body:\n{}", body);
                            Some(body.to_owned())
                        },
                    };

                    match body {
                        Some(body) => {
                            let split = body.split("\n");
                            let lines = split.collect::<Vec<&str>>();
                            let path = lines[0].split(" ").collect::<Vec<&str>>()[1];
                            if DEBUG {
                                println!("first line is {}", lines[0]);
                                println!("path is {}", path);
                            }

                            unsafe {
                                if path != favicon_path {
                                    VISITOR_COUNT += 1;
                                }
                            }

                            let response: std::string::String = match path {
                                p if p == root_path => {
                                    let part1 =
                                    "<doctype !html><html><head><title>Hello, Rust!</title>
                                     <style>body { background-color: #111; color: #FFEEAA }
                                            h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                            h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                                     </style></head>
                                     <body>
                                     <h1>Greetings, Krusty number ";
                                    let part2 = "!</h1></body></html>\r\n";
                                    unsafe {
                                        let resp = format!("{}{}{}{}", ok, part1, VISITOR_COUNT, part2);
                                        resp
                                    }
                                }

                                p if p == favicon_path => {
                                    "".to_string()
                                }

                                some_path => {
                                    let file_name: Option<String> = get_html_file(some_path);
                                    match file_name {
                                        Some(file_name) => {
                                            match File::open(file_name) {
                                                Ok(mut file) => {
                                                    let mut data = String::new();
                                                    match file.read_to_string(&mut data) {
                                                        Err(_) => {
                                                            data = forbidden.to_string();
                                                        },
                                                        Ok(_) => (),
                                                    };
                                                    format!("{}{}", ok, data)
                                                }
                                                Err(_) => forbidden.to_string()
                                            }
                                        }
                                        None => forbidden.to_string()
                                    }
                                }
                            };

                            stream.write(response.as_bytes()).unwrap();
                        }
                        None => {
                            stream.write("Error".as_bytes()).unwrap();
                        }
                    }
                    println!("Connection terminates.");
                });
            },
        }
    }

    drop(listener);
}


fn get_html_file(html_path: &str) -> Option<String> {
    let mut file_name: Option<String> = None;
    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        let dir_path = dir.path();
        if DEBUG {
            println!("Current path is {}", dir_path.display());
            println!("Current file name is {}", dir_path.file_name().unwrap().to_str().unwrap_or(""));
        }

        if dir_path.is_dir() {
            if DEBUG {
                println!("{} is a dir", dir_path.to_str().unwrap());
            }
            continue
        }

        match dir_path.extension() {
            Some(ext) => {
                if ext.to_str().unwrap_or("") != "html" {
                    continue
                }
            }
            None => continue
        }

        if DEBUG {
            println!("found html file");
        }

        println!("{} {}", html_path, dir_path.to_str().unwrap());

        if dir_path.ends_with(html_path[1..].to_string()) {
            println!("file can be served");
            //                                                file_name = Some(&dir_path);
            file_name = Some(dir_path.to_str().unwrap().to_owned());
            break;
        }
    }
    file_name
}