/*
Exercise 2.2. Implement a function, incrementMut that takes as input a vector of integers and
modifies the values of the original list by incrementing each value by one.
*/

fn main() {
    let mut p = vec![1, 2, 3];
    incrementMut(&mut p);
    for &x in p.iter() {
        print!("{} ", x);
    }
}

fn incrementMut(v: &mut Vec<u32>) {
    for i in 0..v.len() {
        v[i] = v[i] + 1;
    }
}