/*
 * locking implementation
 */
use std::io::{self, BufWriter, Write};

fn main() {
    let out = io::stdout();
    let mut handle = out.lock();
    let mut handle = BufWriter::with_capacity(32768, handle);

    for i in 0..u64::MAX {
        let mut s = String::new();
        if i % 3 == 0 {
            s += "fizz";
        }
        if i % 5 == 0 {
            s += "buzz";
        }
        if s == "" {
            s += &i.to_string()[..];
        }
        writeln!(&mut handle, "{}", s);
    }
}
