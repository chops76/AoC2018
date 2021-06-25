use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::cmp;

type Input = (String, HashMap<String, u8>);

fn parse_input(path: &str) -> Input {
	let mut subs = HashMap::new();

	let f = File::open(path).unwrap();
	let mut lines = BufReader::new(f).lines().flatten();

	let initial = lines.next().unwrap().split_whitespace().skip(2).next().unwrap().to_string();
	
	for l in lines.skip(1) {
		let spl = l.split_whitespace().collect::<Vec<&str>>();
		subs.insert(spl[0].to_string(), spl[2].as_bytes()[0]);
	}
	(initial, subs)
}

fn part1(param: Input) -> i32 {
	let (initial, subs) = param;
	let mut work_str = ".........................".to_string() + &initial + ".........................";
	for _ in 0..20 {
		let mut bytes = work_str.clone().into_bytes();
		for i in 0..work_str.len()-4 {
			if subs.contains_key(&work_str[i..i+5]) {
				bytes[i+2] = subs[&work_str[i..i+5]];
			}
		}
		work_str = String::from_utf8(bytes).unwrap();
	}
	let mut sum:i32 = 0;
	for (i, c) in work_str.chars().enumerate() {
		if c == '#' {
			sum += i as i32;
			sum -= 25;
		}
	}
	sum
}

fn part2(param: Input) -> i32 {
	let (initial, subs) = param;
	let mut work_str = ".........................".to_string() + &initial + "................................................................................................................................";
	for count in 0..125 {
		let mut bytes = work_str.clone().into_bytes();
		for i in 0..work_str.len()-4 {
			if subs.contains_key(&work_str[i..i+5]) {
				bytes[i+2] = subs[&work_str[i..i+5]];
			}
		}
		work_str = String::from_utf8(bytes).unwrap();
		let mut sum:i32 = 0;
		for (i, c) in work_str.chars().enumerate() {
			if c == '#' {
				sum += i as i32;
				sum -= 25;
			}
		}
		println!("{}", work_str);
		println!("{}: {}", count, sum);
	}
	// Ultimately, we can see the pattern stabilizes at 120, and from there the value increases by the same
	// amount for each round.  Some easy math will get us the value for the huge round number we're looking for.
	0
}

pub fn main() {
	let (initial, subs) = parse_input("./input/day12/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1((initial.clone(), subs.clone()));
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {:?}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2((initial, subs));
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}