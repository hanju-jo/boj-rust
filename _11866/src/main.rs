// https://www.acmicpc.net/problem/11866
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::StdinLock;
use std::io::Write;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    solve(&mut stdin, &mut stdout);
}

fn solve(stdin: &mut StdinLock, stdout: &mut Write) {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut nm = line.split_whitespace();
    let (n, m): (i32, i32) = (nm.next().unwrap().parse().unwrap(), nm.next().unwrap().parse().unwrap());

    let mut v: VecDeque<i32> = (1..=n).map(|x| x).collect();
    let mut ans: Vec<i32> = Vec::new();

    while !v.is_empty() {
        for i in 1..=m {
            let item = v.pop_front().unwrap();
            if i == m {
                ans.push(item);
            } else {
                v.push_back(item);
            }
        }
    }

    let ans: Vec<String> = ans.iter().map(ToString::to_string).collect();
    write!(stdout, "<");
    write!(stdout, "{}", ans.join(", "));
    writeln!(stdout, ">");
}
