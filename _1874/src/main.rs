// https://www.acmicpc.net/problem/1874
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

    let mut v: Vec<i32> = Vec::new();
    let mut cmd: Vec<&str> = Vec::new();
    let mut j = 1;

    for _i in 1..=n {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a = input.trim().parse().unwrap();

        while j <= a {
            v.push(j);
            j = j + 1;
            cmd.push("+");
        }
        let last = v.last().unwrap_or(&-1).clone();
        if last == a {
            v.pop();
            cmd.push("-");
        }
    }

    if v.is_empty() {
        writeln!(stdout, "{}", cmd.join("\n")).unwrap();
    } else {
        writeln!(stdout, "NO").unwrap();
    }
}
