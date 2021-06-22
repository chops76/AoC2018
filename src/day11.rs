use std::time::Instant;
use std::collections::HashMap;
use std::cmp;

fn part1() -> (usize,usize) {
	let mut grid = Vec::new();
	for y in 1..=300 {
		let mut row = Vec::new();
		for x in 1..=300 {
			let rack_id = x + 10;
			let power_level = (rack_id * y + 1309) * rack_id;
			let digit = (power_level % 1000) / 100;
			row.push(digit as i32 - 5);
		}
		grid.push(row);
	}
	let mut max_spot = (0,0);
	let mut max_val = -1000;
	for y in 0..298 {
		for x in 0..298 {
			let mut sum = 0;
			for y1 in 0..3 {
				for x1 in 0..3 {
					sum += grid[y+y1][x+x1];
				}
			}
			if sum > max_val {
				max_val = sum;
				max_spot = (x+1, y+1);
			}
		}
	}

	max_spot
}

fn part2() -> (usize,usize,usize) {
	let mut grid = Vec::new();
	for y in 1..=300 {
		let mut row = Vec::new();
		for x in 1..=300 {
			let rack_id = x + 10;
			let power_level = (rack_id * y + 1309) * rack_id;
			let digit = (power_level % 1000) / 100;
			row.push(digit as i32 - 5);
		}
		grid.push(row);
	}
	let mut max_spot = (0,0,0);
	let mut max_val = -1000;
	for sq_size in 1..=300 {
		for y in 0..301-sq_size {
			for x in 0..301-sq_size {
				let mut sum = 0;
				for y1 in 0..sq_size {
					for x1 in 0..sq_size {
						sum += grid[y+y1][x+x1];
					}
				}
				if sum > max_val {
					max_val = sum;
					max_spot = (x+1, y+1, sq_size);
				}
			}
		}
	}

	max_spot
}

pub fn main() {
	let p1_timer = Instant::now();
    let p1_result = part1();
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {:?}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {:?}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}