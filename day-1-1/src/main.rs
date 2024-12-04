use std::fs;

fn main() {
    let input  = fs::read_to_string("input.txt").expect("no input file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| -> (i32, i32) {
            let line = line.split("   ").collect::<Vec<&str>>();
            (line[0].parse().unwrap(), line[1].parse().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    let result: i32 = left.into_iter().zip(right.into_iter()).map(|(x, y)| (x - y).abs()).sum();
    println!("{result}");
}
