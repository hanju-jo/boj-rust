// https://www.acmicpc.net/problem/10869
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut s: String = String::new();
    stdin.read_line(&mut s).unwrap();
    let values: Vec<i64> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", values[0] + values[1]);
    println!("{}", values[0] - values[1]);
    println!("{}", values[0] * values[1]);
    println!("{}", values[0] / values[1]);
    println!("{}", values[0] % values[1]);
}