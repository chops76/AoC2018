use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

type Input = Vec<(usize, usize, usize, usize)>;

fn parse_line(s: &str) -> (usize, usize, usize, usize) {
	let spl = s.split("@ ").collect::<Vec<&str>>();
	let coords = spl[1].split(": ").collect::<Vec<&str>>();
	let ul = coords[0].split(",").collect::<Vec<&str>>();
	let size = coords[1].split("x").collect::<Vec<&str>>();
	(ul[0].parse().unwrap(), ul[1].parse().unwrap(), 
	 size[0].parse().unwrap(), size[1].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn make_grid(recs: &Input) -> Vec<Vec<usize>> {
	let mut max_x = 0;
	let mut max_y = 0;
	for r in recs {
		if r.0 + r.2 > max_x {
			max_x = r.0 + r.2;
		}
		if r.1 + r.3 > max_y {
			max_y = r.1 + r.3;
		}
	}
	let mut grid = vec![vec![0; max_x as usize];max_y as usize];
	for r in recs {
		for x in 0..r.2 {
			for y in 0..r.3 {
				grid[r.1 + y][r.0 + x] += 1;
			}
		}
	}

	grid
}

fn part1(recs: &Input) -> usize {
	let grid = make_grid(recs);

	let mut count = 0;
	for x in 0..grid[0].len() {
		for y in 0..grid.len() {
			if grid[y][x] > 1 {
				count += 1;
			}
		}
	}
	count
}

fn part2(recs: &Input) -> usize {
	let grid = make_grid(recs);

	for r in recs.iter().enumerate() {
		let mut found = true;
		for x in 0..r.1.2 {
			for y in 0..r.1.3 {
				if grid[r.1.1 + y][r.1.0 + x] != 1 {
					found = false;
				}
			}
		}
		if found {
			return r.0 + 1;
		}
	}

	0
}

pub fn main() {
    let recs = parse_input("./input/day3/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&recs);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&recs);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}