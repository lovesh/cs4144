/*
The following program finds the number of Collatz steps for from 1 till a given number
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let j : u32 = args[1].parse().ok().expect("Please input integer.");
    for i in 1..j {
        println!("{} has {} Collatz steps", i, collatz(i));
    }

}

pub fn collatz(n: u32) -> u32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3+1) }
    }
}