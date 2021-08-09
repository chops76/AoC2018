use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;
use std::cmp;

fn parse_input(path: &str) -> Vec<Vec<char>> {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| s.chars().collect()).collect()
}

fn part1(orig_grid: &Vec<Vec<char>>) -> usize {
	let mut grid = orig_grid.clone();

	for _ in 0..10 {
		let mut new_grid = grid.clone();
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				let mut num_trees = 0;
				let mut num_lumberyards = 0;
				for xd in 0..=2 {
					for yd in 0..=2 {
						if xd == 1 && yd == 1 {
							continue;
						}
						let new_x = x as i32 + xd as i32 - 1;
						if new_x < 0 || new_x >= grid[y].len() as i32 {
							continue;
						}
						let new_y = y as i32 + yd as i32 - 1;
						if new_y < 0 || new_y >= grid.len() as i32 {
							continue;
						}
						
						if grid[new_y as usize][new_x as usize] == '|' {
							num_trees += 1;
						} else if grid[new_y as usize][new_x as usize] == '#' {
							num_lumberyards += 1;
						}
					}
				}
				
				if grid[y][x] == '.' && num_trees >= 3 {
					new_grid[y][x] = '|';
				} 
				if grid[y][x] == '|' && num_lumberyards >= 3 {
					new_grid[y][x] = '#';
				}
				if grid[y][x] == '#' && (num_trees == 0 || num_lumberyards == 0) {
					new_grid[y][x] = '.';
				}
			}
		}
		grid = new_grid;
	}

	let mut num_trees = 0;
	let mut num_lumberyards = 0;
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] == '|' {
				num_trees += 1;
			}
			if grid[y][x] == '#' {
				num_lumberyards += 1;
			}
			print!("{}", grid[y][x]);
		}
		println!("");
	}

	num_trees * num_lumberyards
}

fn part2(orig_grid: &Vec<Vec<char>>) -> usize {
	let mut grid = orig_grid.clone();

	for minute in 0..10000 {
		let mut new_grid = grid.clone();
		for y in 0..grid.len() {
			for x in 0..grid[y].len() {
				let mut num_trees = 0;
				let mut num_lumberyards = 0;
				for xd in 0..=2 {
					for yd in 0..=2 {
						if xd == 1 && yd == 1 {
							continue;
						}
						let new_x = x as i32 + xd as i32 - 1;
						if new_x < 0 || new_x >= grid[y].len() as i32 {
							continue;
						}
						let new_y = y as i32 + yd as i32 - 1;
						if new_y < 0 || new_y >= grid.len() as i32 {
							continue;
						}
						
						if grid[new_y as usize][new_x as usize] == '|' {
							num_trees += 1;
						} else if grid[new_y as usize][new_x as usize] == '#' {
							num_lumberyards += 1;
						}
					}
				}
				
				if grid[y][x] == '.' && num_trees >= 3 {
					new_grid[y][x] = '|';
				} 
				if grid[y][x] == '|' && num_lumberyards >= 3 {
					new_grid[y][x] = '#';
				}
				if grid[y][x] == '#' && (num_trees == 0 || num_lumberyards == 0) {
					new_grid[y][x] = '.';
				}
			}
		}
		grid = new_grid;
		let mut num_trees = 0;
		let mut num_lumberyards = 0;

		if minute > 9900 {
			for y in 0..grid.len() {
				for x in 0..grid[y].len() {
					if grid[y][x] == '|' {
						num_trees += 1;
					}
					if grid[y][x] == '#' {
						num_lumberyards += 1;
					}
				}
			}
			
			println!("{}: {}", minute+1, num_trees * num_lumberyards);
		}
	}


	// This just had a repeating pattern - a little math gives the right answer without having 
	// to do a ridiculous number of iterations.
	0
}

pub fn main() {
	let grid = parse_input("./input/day18/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&grid);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time); 

	let p2_timer = Instant::now();
    let p2_result = part2(&grid);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time); 
}