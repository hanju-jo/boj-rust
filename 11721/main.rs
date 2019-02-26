// https://www.acmicpc.net/problem/11721
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut out = String::new();
    for (i, c) in buf.chars().enumerate() {
        out.push_str(format!("{}", c).as_str());
        if i % 10 == 9 {
            out.push_str(format!("\n").as_str());
        }
    }
    print!("{}", out);
}