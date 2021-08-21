use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

#[derive(Debug)]
struct Unit {
	num_units: usize,
	hp: usize,
	damage: usize,
	damage_type: String,
	initiative: usize,
	weak: Vec<String>,
	immune: Vec<String>
}

fn parse_input(path: &str) -> (Vec<Unit>, Vec<Unit>) {
	let f = File::open(path).unwrap();
	let mut immune = Vec::new();
	let mut infection = Vec::new();
	let mut group = &mut immune;
	for l in BufReader::new(f).lines().flatten() {
		if l == "Immune System:" || l == "" {
			continue;
		} else if l == "Infection:" {
			group = &mut infection;
			continue;
		}
		let spl = l.split(" with an ").collect::<Vec<&str>>();
		let left = spl[0].split(" ").collect::<Vec<&str>>();
		let right = spl[1].split(" ").collect::<Vec<&str>>();
		let mut new_unit = Unit { num_units: left[0].parse().unwrap(), hp: left[4].parse().unwrap(),
		                          damage: right[3].parse().unwrap(), damage_type: right[4].to_string(),
								  initiative: right[8].parse().unwrap(), weak: Vec::new(), immune: Vec::new() };
		if left.len() != 7 {
			let spl2 = spl[0].split("(").collect::<Vec<&str>>();
			let attribs:&str = &spl2[1][..spl2[1].len()-1];
			for a in attribs.split("; ") {
				let spl = a.split(" to ").collect::<Vec<&str>>();
				for element in spl[1].split(", ") {
					if spl[0] == "weak" {
						new_unit.weak.push(element.to_string());
					} else {
						new_unit.immune.push(element.to_string());
					}
				}
			}
		}
		group.push(new_unit);
	}

	(immune, infection)
}


pub fn main() {
	let (immune, infection) = parse_input("./input/day24/input.txt");
	println!("{:?}", immune);
	println!("{:?}", infection);
/*
	let p1_timer = Instant::now();
    let p1_result = part1(&bots);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time); 

	let p2_timer = Instant::now();
    let p2_result = part2(&bots);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);  */
}