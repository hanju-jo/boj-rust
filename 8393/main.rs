// https://www.acmicpc.net/problem/8393
use std::io;

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    println!("{}", n * (n + 1) / 2);
}