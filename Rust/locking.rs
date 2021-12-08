/*
 * locking implementation
 */
use std::io;

fn main() {
    let out = io::stdout();
    let mut handle = out.lock();

    for i in 0..u64::MAX {
        let mut s = String::new();
        if i % 3 == 0 {
            s += "fizz";
        }
        if i % 5 == 0 {
            s += "buzz";
        }
        if s == "" {
            s += &format!("{}", i)[..];
        }
        println!("{}", s);
    }
}
