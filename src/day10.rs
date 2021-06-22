use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::cmp;

type Input = (Vec<(i32, i32)>, Vec<(i32, i32)>);

fn whitespace_test(c: char) -> bool {
    return c == '<' || c == '>' || c == ',';
}

fn parse_line(s: &str) -> ((i32, i32), (i32, i32)) {
	let spl = s.split(whitespace_test).collect::<Vec<&str>>();
	
	((spl[1].trim_start().parse().unwrap(), spl[2].trim_start().parse().unwrap()), 
	 (spl[4].trim_start().parse().unwrap(), spl[5].trim_start().parse().unwrap()))
}

fn parse_input(path: &str) -> Input {
	let mut pos = Vec::new();
	let mut dir = Vec::new();
	let f = File::open(path).unwrap();
	for l in BufReader::new(f).lines().flatten() {
		let (p, d) = parse_line(&l);
		pos.push(p);
		dir.push(d);
	}
	(pos, dir)
}

fn part1(pos: Vec<(i32, i32)>, dir: Vec<(i32, i32)>) {
	let mut cur_pos = pos.clone();
	for _ in 0..10116 {
		for i in 0..cur_pos.len() {
			cur_pos[i] = (cur_pos[i].0 + dir[i].0, cur_pos[i].1 + dir[i].1);
		}
	}
		
	let mut min_x = 100000;
	let mut max_x = -100000;
	let mut min_y = 100000;
	let mut max_y = -100000;
	for i in 0..cur_pos.len() {
		cur_pos[i] = (cur_pos[i].0 + dir[i].0, cur_pos[i].1 + dir[i].1);
		min_x = cmp::min(min_x, cur_pos[i].0);
		max_x = cmp::max(max_x, cur_pos[i].0);
		min_y = cmp::min(min_y, cur_pos[i].1);
		max_y = cmp::max(max_y, cur_pos[i].1);
	}
	for y in min_y..=max_y {
		for x in min_x..=max_x {
			if cur_pos.contains(&(x, y)) {
				print!("*");
			} else {
				print!(" ");
			}
		}
		println!("");
	}
}

pub fn main() {
	let (pos, dir) = parse_input("./input/day10/input.txt");

	let p1_timer = Instant::now();
    part1(pos, dir);
    let p1_time = p1_timer.elapsed();
	println!("Part 1 Time: {:?}", p1_time);
/*
	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time); */
}