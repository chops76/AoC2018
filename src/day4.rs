use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

type Input = HashMap<(i32, String), Vec<usize>>;

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	let mut lines = BufReader::new(f).lines().flatten().collect::<Vec<String>>();
	lines.sort();
	
	let mut guards = HashMap::new();
	let mut cur_guard = -1;
	let mut date = String::new();
	let mut vals = Vec::new();
	for line in lines {
		let spl = line.split(" ").collect::<Vec<&str>>();
		if spl[2] == "Guard" {
			if cur_guard != -1 {
				guards.insert((cur_guard, date.clone()), vals);
				vals = Vec::new();
			}
			cur_guard = spl[3][1..].parse::<i32>().unwrap();
		} else {
			date = spl[0][1..].to_string();
			vals.push(spl[1][3..5].parse::<usize>().unwrap());
		}
	}
	
	guards
}

fn create_map(guards: &Input) -> HashMap<i32, Vec<Vec<bool>>> {
	let mut hm:HashMap<i32, Vec<Vec<bool>>> = HashMap::new();
	for guard in guards {
		let mut cur_pos = 0;
		let mut asleep = false;
		let mut minutes = vec!(false; 60);
		for val in guard.1 {
			for i in cur_pos..*val {
				minutes[i] = asleep;
				cur_pos = *val;
			}
			asleep = !asleep;
		}
		for i in cur_pos..60 {
			minutes[i] = asleep;
		}
		let mut v = Vec::new();
		if hm.contains_key(&guard.0.0) {
			v = hm[&guard.0.0].clone();
		}
		v.push(minutes);
		hm.insert(guard.0.0, v);
	}

	hm
}

fn part1(guards: &Input) -> usize {
	let hm = create_map(guards);

	let mut best_guard = 0;
	let mut most_asleep = 0;
	for guard in &hm {
		let mut count = 0;
		for day in guard.1 {
			count += day.iter().filter(|m| **m).count();
		}
		if count > most_asleep {
			most_asleep = count;
			best_guard = *guard.0;
		}
	}

	let (max_idx, _) = max_minute(&hm[&best_guard]);
	
	best_guard as usize * max_idx
}

fn part2(guards: &Input) -> usize {
	let hm = create_map(guards);

	let mut best_guard = 0;
	let mut best_minute = 0;
	let mut max_asleep = 0;
	for guard in &hm {
		let (max_idx, max_min) = max_minute(&guard.1);
		if max_min > max_asleep {
			best_minute = max_idx;
			best_guard = *guard.0;
			max_asleep = max_min;
		}
	}
	
	best_guard as usize * best_minute
}

fn max_minute(vals: &Vec<Vec<bool>>) -> (usize, usize) {
	let mut minutes = vec!(0; 60);
	for day in vals {
		for i in 0..60 {
			if day[i] {
				minutes[i] += 1;
			}
		}
	}
	let mut max_min = 0;
	let mut max_idx = 0;
	for i in 0..minutes.len() {
		if minutes[i] > max_min {
			max_min = minutes[i];
			max_idx = i;
		}
	}
	(max_idx, max_min)
}

pub fn main() {
    let guards = parse_input("./input/day4/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&guards);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&guards);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}