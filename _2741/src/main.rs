// https://www.acmicpc.net/problem/2741
use std::io::{self, Write, BufWriter};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    let out = io::stdout();
    let mut bufout = BufWriter::new(out.lock());

    for i in 1..=n {
        write!(bufout, "{}\n", i).unwrap();
    }
}