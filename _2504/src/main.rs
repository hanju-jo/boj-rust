// https://www.acmicpc.net/problem/2504
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let scores: HashMap<u8, i32> = vec![(b'(', 2), (b')', 2), (b'[', 3), (b']', 3) ]
        .into_iter().collect();
    let pairs: HashMap<u8, u8> = vec![(b'(', b')'), (b')', b'('), (b'[', b']'), (b']', b'[') ]
        .into_iter().collect();

    let mut stack = Vec::new();
    let mut previous: u8 = b'_';
    let mut answer: i32 = 0;
    let mut current: i32 = 1;

    for b in input.trim().bytes() {
        match b {
            b'(' | b'['  => {
                stack.push(b);
                current = current * scores.get(&b).unwrap();
            },
            b')' | b']'  => {
                if (stack.is_empty()) || (stack.last() != pairs.get(&b)) {
                    answer = 0;
                    break;
                }
                if previous == *pairs.get(&b).unwrap() {
                    answer = answer + current;
                }
                stack.pop();
                current = current / scores.get(&b).unwrap();
            },
            _ => {}
        }
        previous = b;
    }

    if !stack.is_empty() {
        answer = 0;
    }
    writeln!(stdout, "{}", answer);
}
