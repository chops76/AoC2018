use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().collect()
}

fn get_counts(s: &str) -> (bool, bool) {
	let mut hm = HashMap::new();
	for c in s.chars() {
		if hm.contains_key(&c) {
			hm.insert(c, hm[&c] + 1);
		} else {
			hm.insert(c, 1);
		}
	}
	let mut has_2 = false;
	let mut has_3 = false;
	for (_, n) in hm {
		if n == 2 {
			has_2 = true;
		} else if n == 3 {
			has_3 = true;
		}
	}
	(has_2, has_3)
}

fn part1(strings: &Input) -> usize {
	let mut count_2 = 0;
	let mut count_3 = 0;
	for s in strings {
		let (has_2, has_3) = get_counts(s);
		if has_2 {
			count_2 += 1;
		}
		if has_3 {
			count_3 += 1;
		}
	}
	count_2 * count_3
}

fn matches(s1: &str, s2: &str) -> String {
	for i in 0..s1.len() {
		if s1.chars().enumerate().filter(|(n, _)| *n != i).eq(
			s2.chars().enumerate().filter(|(n, _)| *n != i)) {
				return s1.chars().enumerate().filter(|(n, _)| *n != i).map(|(_, c)| c).collect();
			}
	}
	String::new()
}

fn part2(strings: &Input) -> String {
	for i in 0..strings.len() - 1 {
		for j in i+1 .. strings.len() {
			let tmp = matches(&strings[i], &strings[j]);
			if tmp != String::new() {
				return tmp;
			}
		}
	}
	String::new()
}

pub fn main() {
    let strings = parse_input("./input/day2/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&strings);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&strings);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}