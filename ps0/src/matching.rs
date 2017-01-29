/*
Given a tuple containing an int and a bool, use a match statement to determine
(a) if the bool is true and the int is between 20 and 26,
(b) if the bool is true and the aforementioned condition isn't true for the int,
(c) if the int is between 40 and 49 (where the value of the bool doesn't matter), and
(d), wherein none of the previous conditions are true
*/

fn main() {
    fn mch((i, b): (u32, bool)) {
        match (i, b) {
            (i, b) if i >= 20 && i <= 26 && b  => { println!("{}: Number between 20 and 26 and boolean true", i); }
            (i, _) if i >= 40 && i <= 49 => { println!("{}: Number between 40 and 49 and whatever boolean", i); }
            (i, b) if i < 20 || i > 26 && b => { println!("{}: Number not between 20 and 26 and boolean true", i); }
            _              => { println!("{} {}: Who cares", i, b)}
        }
    }
    let ib1 = (21, true);
    let ib2 = (1, true);
    let ib3 = (43, true);
    let ib4 = (44, false);
    let ib5 = (22, false);

    mch(ib1);
    mch(ib2);
    mch(ib3);
    mch(ib4);
    mch(ib5);
}