// https://www.acmicpc.net/problem/2164
use std::io;
use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    solve(n);
}

fn solve(n: i32) {
    let mut q: VecDeque<i32> = (1..=n).collect::<VecDeque<i32>>();
    let mut flag: bool = true;
    while q.len() > 1 {
        if flag {
            q.pop_front();
            flag = false;
        } else {
            let value = q.pop_front().unwrap();
            q.push_back(value);
            flag = true;
        }
    }
    println!("{}", q.pop_front().unwrap());
}