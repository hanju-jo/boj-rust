// https://www.acmicpc.net/problem/9012
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    for _i in 1..=n {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        writeln!(stdout, "{}", if solve(&input) {"YES"} else {"NO"}).unwrap();
    }

}

fn solve(input: &String) -> bool {
    let mut stack = Vec::new();
    for b in input.trim().bytes() {
        match b {
            b'(' => stack.push(b),
            b')' => {
                if stack.is_empty() {
                    return false
                }
                stack.pop();
            },
            _ => {}
        }
    }
    stack.is_empty()
}
