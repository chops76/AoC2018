use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp;

type Input = Vec<i32>;

fn parse_input(path: &str) -> Input {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_node(start: &mut usize, data: &Input) -> i32 {
	let num_nodes = data[*start];
	let num_datas = data[*start+1];
	let mut total = 0;
	*start += 2;
	for _ in 0..num_nodes {
		total += parse_node(start, data);
	}
	for _ in 0..num_datas {
		total += data[*start];
		*start += 1;
	}
	total
}

fn part1(data: &Input) -> i32 {
	let mut start = 0;
	parse_node(&mut start, data)
}

fn parse_node2(start: &mut usize, data: &Input) -> i32 {
	let num_nodes = data[*start];
	let num_datas = data[*start+1];
	let mut total = 0;
	let mut node_totals = Vec::new();
	*start += 2;
	for _ in 0..num_nodes {
		node_totals.push(parse_node2(start, data));
	}
	if num_nodes == 0 {
		for _ in 0..num_datas {
			total += data[*start];
			*start += 1;
		}
	} else {
		for _ in 0..num_datas {
			if data[*start] != 0 && data[*start] <= node_totals.len() as i32 {
				total += node_totals[data[*start] as usize - 1];
			}
			*start += 1;
		}
	}

	total
}

fn part2(data: &Input) -> i32 {
	let mut start = 0;
	parse_node2(&mut start, data)
}

pub fn main() {
    let data = parse_input("./input/day8/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&data);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&data);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time); 
}