// https://www.acmicpc.net/problem/1158
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

    solve(&mut stdin, &mut stdout);
}

fn solve(stdin: &mut StdinLock, stdout: &mut Write) {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut nm = line.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() );
    let (n, m): (usize, usize) = (nm.next().unwrap(), nm.next().unwrap()-1);

    let mut v = (1..=n).collect::<Vec<usize>>();
    let mut ans = Vec::new();
    let mut i = 0;
    while !v.is_empty() {
        i = (i + m) % v.len();
        ans.push(v.remove(i));
    }

    let ans: Vec<String> = ans.iter().map(ToString::to_string).collect();
    write!(stdout, "<");
    write!(stdout, "{}", ans.join(", "));
    writeln!(stdout, ">");
}
