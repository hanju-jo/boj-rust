// https://www.acmicpc.net/problem/1008
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut s: String = String::new();
    stdin.read_line(&mut s).unwrap();
    let values: Vec<f64> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", values[0] / values[1]);
}