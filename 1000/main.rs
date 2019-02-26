// https://www.acmicpc.net/problem/1000
use std::io::stdin;

fn main() {
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();
    let values: Vec<i8> = s.split_whitespace().map(|s: _| s.parse().unwrap()).collect();
    println!("{}", values[0] + values[1]);
}