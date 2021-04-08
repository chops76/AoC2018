use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

fn parse_input(path: &str) -> String {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr
}

fn calc_val(unit_vec_orig: &VecDeque<char>) -> usize {
	let mut unit_vec = unit_vec_orig.clone();
	let mut length = unit_vec.len();
	//let mut cur_pos = 0;
	loop {
		let mut found = false;
		for i in 0..length - 1 {
			//if i > cur_pos {
			//	cur_pos = i;
			//}
			if (unit_vec[i].is_uppercase() && unit_vec[i].to_ascii_lowercase() == unit_vec[i+1]) ||
			   (unit_vec[i].is_lowercase() && unit_vec[i].to_ascii_uppercase() == unit_vec[i+1]) {
				unit_vec.remove(i+1);
				unit_vec.remove(i);
				found = true;
				break;
			}
		}
		if !found {
			break;
		}
		length -= 2;
	}
	
	unit_vec.len()
}

fn part1(units: &str) -> usize {
	let unit_vec = units.chars().collect::<VecDeque<char>>();
	calc_val(&unit_vec)
}

fn part2(units: &str) -> usize {
	let mut smallest = units.len();
	for c in 'a'..'z' {
		let unit_vec = units.chars().filter(|chr| *chr != c && *chr != c.to_ascii_uppercase()).collect::<VecDeque<char>>();
		let tmp = calc_val(&unit_vec);
		if tmp < smallest {
			smallest = tmp;
		}
	}

	smallest
}

pub fn main() {
    let units = parse_input("./input/day5/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&units);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&units);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}