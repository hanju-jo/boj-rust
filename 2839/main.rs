// https://www.acmicpc.net/problem/2839
use std::io;

fn main() {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut array: [i32; 5011] = [-1; 5011];

    array[1] = -1;
    array[2] = -1;
    array[3] = 1;
    array[4] = -1;
    array[5] = 1;
    array[6] = 2;
    array[7] = -1;
    array[8] = 2;
    array[9] = 3;


    for i in 6..(n + 1) {
        match (array[(i - 3) as usize], array[(i - 5) as usize]) {
            (-1, -1) => array[i as usize] = -1,
            (-1, _) => array[i as usize] = array[(i - 5) as usize] + 1,
            (_, -1) => array[i as usize] = array[(i - 3) as usize] + 1,
            (_, _) => array[i as usize] = std::cmp::min(array[(i - 3) as usize], array[(i - 5) as usize]) + 1,
        }
    }
    println!("{}", array[n as usize]);
}