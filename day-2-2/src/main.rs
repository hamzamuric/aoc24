use std::fs;

fn all_increasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|pair| {
        let &[a, b] = pair else { return false };
        a - b > 0 && a - b <= 3
    })
}

fn all_decreasing(nums: &[i32]) -> bool {
    nums.windows(2).all(|pair| {
        let &[b, a] = pair else { return false };
        a - b > 0 && a - b <= 3
    })
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input file");
    let result = input
        .lines()
        .map(|line| line.split(' ')
            .map(|numeral| numeral.parse().unwrap()).collect::<Vec<i32>>()
        )
        .filter(|nums| {
            for i in 0..nums.len() {
                let mut nums = nums.clone();
                nums.remove(i);
                if all_increasing(&nums) || all_decreasing(&nums) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("{result}");
}
