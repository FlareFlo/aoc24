use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    let sol = fs::read_to_string("data/01.txt").unwrap();
    println!("p1: {}", solution(&sol));
    println!("p2: {}", solution_part_2(&sol));
}

fn solution(input: &str) -> u64 {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| line.split_once("   "))
        .flatten()
        .map(|(left, right)| (u64::from_str(left).unwrap(), u64::from_str(right).unwrap()))
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn solution_part_2(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|line| line.split_once("   "))
        .flatten()
        .map(|(left, right)| (u64::from_str(left).unwrap(), u64::from_str(right).unwrap()))
        .collect_vec();
    let mut weight = HashMap::with_capacity(input.len());
    for &(_, num) in &input {
        *weight.entry(num).or_insert(0) += 1;
    }

    input
        .iter()
        .map(|(left, _)| {
            if let Some(weight) = weight.get(left) {
                left * weight
            } else {
                0
            }
        })
        .sum()
}
