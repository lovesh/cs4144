/*
Exercise 2.1. Implement a function, increment that takes as input a vector of integers and
returns a new vector of integers that has the values of the original list each
incremented by one.
*/


fn main() {
    let p = vec![1, 2, 3];
    let q = increment(&p);
    for &x in q.iter() {
        print!("{} ", x);
    }
}

fn increment(v: &Vec<u32>) -> Vec<u32> {
    let mut n: Vec<u32> = Vec::with_capacity(v.len());
    for &x in v.iter() {
        n.push(x+1)
    }
    n
}