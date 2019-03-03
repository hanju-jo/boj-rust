// https://www.acmicpc.net/problem/11720
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let mut bufs = buf.split_whitespace();
    let _: u32 = bufs.next().unwrap().parse().unwrap();
    let sum: u32 = bufs.next().unwrap().chars()
        .map(|x: _| x.to_digit(10).unwrap())
        .collect::<Vec<_>>().iter().sum();
    println!("{}", sum);
}