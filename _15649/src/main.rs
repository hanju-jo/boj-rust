// https://www.acmicpc.net/problem/15649
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let mut nm = line.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() );
    let (n, m): (usize, usize) = (nm.next().unwrap(), nm.next().unwrap());

    solve(n, m);
}

fn solve(n: usize, m: usize) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut v: Vec<usize> = Vec::new();
    let mut visited: [bool; 10] = [false; 10];
    dfs(n, m, &mut v, &mut visited, &mut stdout);
}

fn dfs(n: usize, m: usize, v: &mut Vec<usize>, visited: &mut [bool], stdout: &mut Write) {
    if v.len() == m {
        let vs: Vec<String> = v.iter().map(ToString::to_string).collect();
        writeln!(stdout, "{}", vs.join(" "));
    }

    for i in 1..=n {
        if !visited[i] {
            v.push(i);
            visited[i] = true;
            dfs(n, m, v, visited, stdout);
            v.pop();
            visited[i] = false;
        }
    }
}
