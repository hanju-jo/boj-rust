// https://www.acmicpc.net/problem/2739
use std::io;

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    for i in 1..10 {
        println!("{} * {} = {}", n, i, n * i);
    }
}