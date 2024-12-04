use std::{collections::HashMap, fs};

fn main() {
    let input  = fs::read_to_string("input.txt").expect("no input file");
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| -> (i32, i32) {
            let line = line.split("   ").collect::<Vec<&str>>();
            (line[0].parse().unwrap(), line[1].parse().unwrap())
        })
        .unzip();
    
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for x in right.into_iter() {
        *counts.entry(x).or_default() += 1;
    }

    let result: i32 = left.iter().map(|x|  x * counts.get(x).unwrap_or(&0)).sum();
    println!("{result}");
}
