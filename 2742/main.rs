// https://www.acmicpc.net/problem/2742
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let mut out = String::new();

    for i in (0..n).rev() {
        out.push_str(format!("{}\n", i + 1).as_str());
    }
    println!("{}", out);
}