use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input file");
    let result = input
        .lines()
        .map(|line| line.split(' ')
            .map(|numeral| numeral.parse().unwrap()).collect::<Vec<i32>>()
        )
        .filter(|nums| {
            let all_increasing = nums.windows(2).all(|pair| {
                let &[a, b] = pair else { return false };
                a - b > 0 && a - b <= 3
            });
            let all_decreasing = nums.windows(2).all(|pair| {
                let &[b, a] = pair else { return false };
                a - b > 0 && a - b <= 3
            });

            all_increasing || all_decreasing
        })
        .count();

    println!("{result}");
}
