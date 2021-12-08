/*
 * naive implementation
 */

fn main() {

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
