// https://www.acmicpc.net/problem/1966
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::StdinLock;
use std::io::Write;

fn solve(stdin: &mut StdinLock, stdout: &mut Write) {
    let mut t = String::new();
    stdin.read_line(&mut t).unwrap();

    for _ in 1..=t.trim().parse().unwrap() {
        let mut nm_line = String::new();
        stdin.read_line(&mut nm_line).unwrap();
        let mut nm = nm_line.split_whitespace();
        let (_n, m): (u8, usize) = (
            nm.next().unwrap().parse().unwrap(), nm.next().unwrap().parse().unwrap());

        let mut v_line = String::new();
        stdin.read_line(&mut v_line).unwrap();

        let mut q: VecDeque<(u8, bool)> = v_line
            .split_whitespace()
            .map(|d| d.parse::<u8>().unwrap())
            .enumerate()
            .map(|v| (v.1, v.0 == m))
            .collect();

        let mut ans = 0;
        loop {
            let max = *q.iter()
                .map(|(v, _)| v)
                .max()
                .unwrap();
            let (v, is_target) = q.pop_front().unwrap();
            if max > v {
                q.push_back((v, is_target));
            } else {
                ans += 1;
                if is_target {
                    writeln!(stdout, "{}", ans);
                    break;
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    solve(&mut stdin, &mut stdout);
}
