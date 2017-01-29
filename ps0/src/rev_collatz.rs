/*
The following program finds the minimum number for a given number of Collatz steps.
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let i : u32 = args[1].parse().ok().expect("Please input integer.");
    println!("{} is the minimum number with {} Collatz steps", rev_collatz(i), i);
}


fn rev_collatz(n: u32) -> u32 {
    let mut prev = vec![2];
    let mut cur = vec![];
    for i in 2..n+1 {
        cur = vec![];
        for &x in &mut prev.iter() {
            cur.push(2*x);
            let (q, r) = ((x-1) / 3, (x-1) % 3);
            if r == 0 && q != 1 && q % 2 == 1 {
                cur.push(q);
            }
        }
        prev = cur.clone();
    }
    match cur.iter().min() {
        Some(&m) => {m}
        None => panic!("how come steps[] is empty")
    }
}
