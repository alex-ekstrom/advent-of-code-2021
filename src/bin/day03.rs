use itertools::Itertools;
use std::iter;

fn parse(data: &str) -> (usize, Vec<u32>) {
    let vals = data
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();
    (data.lines().nth(0).unwrap().chars().count(), vals)
}

fn most_common_bit(nums: &[u32], bit: usize) -> u32 {
    let dist = nums
        .into_iter()
        .map(|n| (n >> bit) & 1)
        .fold(0, |c, n| match n {
            1 => c + 1,
            0 => c - 1,
            _ => unreachable!(),
        });
    (dist >= 0) as u32
}

fn solve_a((bits, data): &(usize, Vec<u32>)) -> u32 {
    let x = (0..*bits)
        .map(|i| most_common_bit(data, i) << i)
        .sum::<u32>();
    x * (!x
        & iter::repeat(1)
            .take(*bits)
            .enumerate()
            .map(|(i, bit)| bit << i)
            .sum::<u32>())
}

fn sift_data(nums: &[u32], bit: i32, oxygen: u32) -> u32 {
    if (nums.len() == 1 || bit < -1) {
        return nums[0];
    };

    let mcb = most_common_bit(nums, bit as usize) ^ oxygen;
    sift_data(
        &nums
            .iter()
            .cloned()
            .filter(|n| (*n >> bit) & 1 == mcb)
            .collect::<Vec<_>>(),
        bit - 1,
        oxygen,
    )
}

fn solve_b((bits, data): &(usize, Vec<u32>)) -> u32 {
    sift_data(data, (*bits - 1) as i32, 0) * sift_data(data, (*bits - 1) as i32, 1)
}

aoc2021::main!(3);
