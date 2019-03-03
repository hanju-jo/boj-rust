// https://www.acmicpc.net/problem/1924
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let bufs: Vec<i32> = buf
        .split_whitespace()
        .map(|s: _| s.parse().unwrap())
        .collect();

    calculate_days(bufs[0], bufs[1]);
}

fn calculate_days(month: i32, date: i32) {
    let mut days: i32 = 0;
    for i in 1..month {
        days = days + match i {
            2 => 28,
            4 | 6 | 9 | 10 => 30,
            _ => 31,
        };
    }
    days = days + date;

    match days % 7 {
        0 => println!("SUN"),
        1 => println!("MON"),
        2 => println!("TUE"),
        3 => println!("WED"),
        4 => println!("THU"),
        5 => println!("FRI"),
        _ => println!("SAT"),
    }
}