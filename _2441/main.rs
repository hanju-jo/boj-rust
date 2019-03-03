// https://www.acmicpc.net/problem/2441
use std::io;

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let mut out: String = String::new();
    for i in 1..(n + 1) {
        for _ in 1..i {
            out.push_str(format!(" ").as_str());
        }
        for _ in 0..(n - i + 1) {
            out.push_str(format!("*").as_str());
        }
        out.push_str(format!("\n").as_str());
    }
    println!("{}", out);
}