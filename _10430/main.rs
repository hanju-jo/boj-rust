// https://www.acmicpc.net/problem/10430
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut s: String = String::new();
    stdin.read_line(&mut s).unwrap();
    let values: Vec<i64> = s
        .split_whitespace()
        .map(|s: _| s.parse().unwrap())
        .collect();

    let a: i64 = values[0];
    let b: i64 = values[1];
    let c: i64 = values[2];
    println!("{}", (a + b) % c);
    println!("{}", (a % c + b % c) % c);
    println!("{}", (a * b) % c);
    println!("{}", (a % c * b % c) % c);
}