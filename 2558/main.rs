// https://www.acmicpc.net/problem/2558
use std::io;

fn main() {
    let a: i32 = read_number();
    let b: i32 = read_number();

    println!("{}", a + b);
}

fn read_number() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    return input.trim()
        .parse::<i32>()
        .expect("Input is not an integer");
}
