// https://www.acmicpc.net/problem/10828
use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let n = line.trim().parse().unwrap();

    let mut stack: Stack<i32> = Stack::new();
    for _i in 1..=n {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        execute(&mut stack, &line, &mut stdout);
    }
}

fn execute(stack: &mut Stack<i32>, command_line: &String, stdout: &mut Write) {
    let mut command = command_line.split_whitespace();
    match command.next() {
        Some(s) => match s {
            "push" => {
                let x: i32 = command.next().unwrap().parse::<i32>().unwrap();
                stack.push(x);
            }
            "pop" => writeln!(stdout, "{}", stack.pop().unwrap_or(-1)).unwrap(),
            "size" => writeln!(stdout, "{}", stack.size()).unwrap(),
            "empty" => writeln!(stdout, "{}", if stack.empty() {1} else {0}).unwrap(),
            "top" => writeln!(stdout, "{}", stack.top().unwrap_or(&-1)).unwrap(),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

struct Stack<T> {
    contents: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            contents: Vec::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.contents.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }
    fn size(&mut self) -> usize {
        self.contents.len()
    }
    fn empty(&mut self) -> bool {
        self.contents.is_empty()
    }
    fn top(&mut self) -> Option<&T> {
        self.contents.last()
    }
}