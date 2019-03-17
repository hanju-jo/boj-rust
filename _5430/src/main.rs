// https://www.acmicpc.net/problem/5430
use std::collections::VecDeque;

use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::StdinLock;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();
    for _i in 0..t {
        solve(&mut stdin, &mut stdout);
    }
}

fn solve(stdin: &mut StdinLock, stdout: &mut Write) {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    stdin.read_line(&mut line).unwrap();
    stdin.read_line(&mut line).unwrap();
    let mut lines = line.lines();
    let ps: Vec<char> = lines.next().unwrap().chars().collect();
    let n: i32 = lines.next().unwrap().trim().parse().unwrap();

    let mut v: VecDeque<u8> = VecDeque::with_capacity(100011);
    if n > 0 {
        let mut xs = lines.next().unwrap();
        unsafe {
            xs = xs.get_unchecked(1..xs.len()-1);
        }
        v = xs.split(",")
            .map(|x: _| x.parse().unwrap())
            .collect();
    }

    let mut flag: bool = true;

    for p in ps {
        match p {
            'R' => {
                flag = !flag;
            },
            'D' => {
                if v.is_empty() {
                    writeln!(stdout, "error");
                    return
                } else {
                    if flag {
                        v.pop_front();
                    } else {
                        v.pop_back();
                    }
                }
            },
            _ => unreachable!()
        }
    }

    let mut ans: Vec<String> = Vec::with_capacity(100011);

    if flag {
        ans = v.iter()
            .map(ToString::to_string)
            .collect();
    } else {
        ans = v.iter().rev()
            .map(ToString::to_string)
            .collect();
    }
    writeln!(stdout, "[{}]", ans.join(","));
    stdout.flush();
}