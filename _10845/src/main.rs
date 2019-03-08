// https://www.acmicpc.net/problem/10828
use std::io;
use std::io::BufWriter;
use std::io::BufRead;
use std::collections::VecDeque;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let n = line.trim().parse().unwrap();

    let mut queue = Queue::new();
    for _i in 1..=n {
        line.clear();
        stdin.read_line(&mut line).unwrap();

        execute(&mut queue, &line, &mut writer);
    }
}

fn execute(queue: &mut Queue<i32>, cmd_line: &String, writer: &mut Write) {
    let mut cmd = cmd_line.split_whitespace();
    match cmd.next() {
        Some(s) => match s {
            "push" => {
                let x: i32 = cmd.next().unwrap().parse().unwrap();
                queue.push(x);
            },
            "pop" => writeln!(writer, "{}", queue.pop().unwrap_or(-1)).unwrap(),
            "size" => writeln!(writer, "{}", queue.size()).unwrap(),
            "empty" => writeln!(writer, "{}", if queue.empty() {1} else {0}).unwrap(),
            "front" => writeln!(writer, "{}", queue.front().unwrap_or(&-1)).unwrap(),
            "back" => writeln!(writer, "{}", queue.back().unwrap_or(&-1)).unwrap(),
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

struct Queue<T> {
    contents: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            contents: VecDeque::new(),
        }
    }
    fn push(&mut self, value: T) {
        self.contents.push_back(value);
    }
    fn pop(&mut self) -> Option<T> {
        self.contents.pop_front()
    }
    fn size(&mut self) -> usize {
        self.contents.len()
    }
    fn empty(&mut self) -> bool {
        self.contents.is_empty()
    }
    fn front(&mut self) -> Option<&T> {
        self.contents.front()
    }
    fn back(&mut self) -> Option<&T> {
        self.contents.back()
    }

}
