/*
The Collatz Conjecture states that, starting from any natural number, it is possible to reach 1 by
following certain rules:
Take n:
1. If n is odd, set n = 3n+1.
2. Else, if n is even, set n = n/2.
Repeat the procedure until 1 is reached.
The following program finds the number of Collatz steps for a given number
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let i : u32 = args[1].parse().ok().expect("Please input integer.");
    println!("{} has {} Collatz steps", i, collatz(i));
}

pub fn collatz(n: u32) -> u32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3+1) }
    }
}
