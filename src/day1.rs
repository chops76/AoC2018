use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

fn parse_input(path: &str) -> Vec<i32> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn part1(vals: &Vec<i32>) -> i32 {
    vals.iter().sum()
}

fn part2(vals: &Vec<i32>) -> i32 {
	let mut seen = HashSet::new();
	let mut cur_sum = 0;
	for i in vals.iter().cycle() {
		if seen.contains(&cur_sum) {
			return cur_sum;
		}
		seen.insert(cur_sum);
		cur_sum += i;
	}
	0
}

pub fn main() {
    let vals = parse_input("./input/day1/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&vals);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&vals);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}