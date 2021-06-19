use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::cmp;

type Input = Vec<(String, String)>;

fn parse_line(s: &str) -> (String, String) {
	let spl = s.split_whitespace().collect::<Vec<&str>>();
	(spl[1].to_string(), spl[7].to_string())
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(nodes: &Input) -> String {
	let mut reqs = HashMap::new();
	let mut satisfied = HashMap::new();
	for n in nodes {
		if !satisfied.contains_key(&n.0) {
			satisfied.insert(n.0.clone(), false);
		}
		if !satisfied.contains_key(&n.1) {
			satisfied.insert(n.1.clone(), false);
		}

		if !reqs.contains_key(&n.1) {
			let mut v = Vec::new();
			v.push(n.0.clone());
			reqs.insert(n.1.clone(), v);
		} else {
			let mut v = reqs[&n.1].clone();
			v.push(n.0.clone());
			reqs.insert(n.1.clone(), v);
		}
	}
	for s in &satisfied {
		if !reqs.contains_key(s.0) {
			reqs.insert(s.0.clone(), Vec::new());
		}
	}

	let mut sequence = String::new();
	loop {
		let mut ready = Vec::new();
		for r in &reqs {
			if satisfied[r.0] {
				continue;
			}
			let mut ok = true;
			for rule in r.1 {
				if !satisfied[rule] {
					ok = false;
					break;
				}
			}
			if ok {
				ready.push(r.0.clone());
			}
		}
		if ready.len() == 0 {
			break;
		}
		ready.sort();
		satisfied.insert(ready[0].clone(), true);
		sequence += &ready[0];
	}

	sequence
}

fn part2(nodes: &Input) -> usize {
	let mut reqs = HashMap::new();
	let mut satisfied = HashMap::new();
	let mut active = HashMap::new();
	for n in nodes {
		if !satisfied.contains_key(&n.0) {
			satisfied.insert(n.0.clone(), false);
		}
		if !satisfied.contains_key(&n.1) {
			satisfied.insert(n.1.clone(), false);
		}

		if !reqs.contains_key(&n.1) {
			let mut v = Vec::new();
			v.push(n.0.clone());
			reqs.insert(n.1.clone(), v);
		} else {
			let mut v = reqs[&n.1].clone();
			v.push(n.0.clone());
			reqs.insert(n.1.clone(), v);
		}
	}
	for s in &satisfied {
		if !reqs.contains_key(s.0) {
			reqs.insert(s.0.clone(), Vec::new());
		}
	}

	let mut sequence = String::new();
	let mut secs = 0;
	loop {
		let mut ready = Vec::new();
		for r in &reqs {
			if satisfied[r.0] {
				continue;
			}
			let mut ok = true;
			for rule in r.1 {
				if !satisfied[rule] {
					ok = false;
					break;
				}
			}
			if ok {
				ready.push(r.0.clone());
			}
		}

		ready.sort();
		let mut idx = 0;
		while active.len() < 5 && idx < ready.len() {
			if !active.contains_key(&ready[idx]) {
				active.insert(ready[idx].clone(), 61 + ready[idx].bytes().nth(0).unwrap() - "A".bytes().nth(0).unwrap());
				sequence += &ready[idx];
			}
			idx+=1;
		}

		if active.len() == 0 {
			break;
		}
		let cloned = active.clone();
		let keys = cloned.keys();
		for key in keys {
			let new_val = active[key] - 1;
			if new_val == 0 {
				active.remove(key);
				satisfied.insert(key.clone(), true);
			} else {
				active.insert(key.clone(), new_val);
			}
		}
		secs += 1;
	}

	secs
}

pub fn main() {
    let nodes = parse_input("./input/day7/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&nodes);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&nodes);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}