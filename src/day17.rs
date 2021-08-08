use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;
use std::cmp;

fn parse_input(path: &str) -> Vec<Vec<char>> {
	let mut grid = vec![vec!['.';2000];2000];

	let f = File::open(path).unwrap();
	let lines = BufReader::new(f).lines().flatten();

	for l in lines {
		let spl = l.split(", ").collect::<Vec<&str>>();
		let left_spl = spl[0].split("=").collect::<Vec<&str>>();
		let const_coord:usize = left_spl[1].parse().unwrap();
		let right_spl = spl[1].split("=").collect::<Vec<&str>>();
		let range_spl = right_spl[1].split("..").collect::<Vec<&str>>();
		let lower_bound:usize = range_spl[0].parse().unwrap();
		let upper_bound:usize = range_spl[1].parse().unwrap();

		if left_spl[0] == "x" {
			for y in lower_bound..=upper_bound {
				grid[const_coord][y] = '#';
			}
		} else {
			for x in lower_bound..=upper_bound {
				grid[x][const_coord] = '#';
			}
		}
	}

	grid
}

fn part1(grid: &Vec<Vec<char>>) -> (usize,usize) {
	let mut sources = HashSet::new();
	let mut grid = grid.clone();
	let mut min_y = 2000;
	let mut max_y = 0;
	for x in 0..2000 {
		for y in 0..2000 {
			if grid[x][y] == '#' {
				min_y = cmp::min(y, min_y);
				max_y = cmp::max(y, max_y);
			}
		}
	}
	sources.insert((500,0));
	while sources.len() != 0 {
		let source = sources.iter().next().unwrap().clone();
   		sources.remove(&source);
		
		let x = source.0;
		let mut y = source.1;
		
		while grid[x][y + 1] == '.' && y < max_y {
			grid[x][y] = '|';
			y += 1;

		}
		if y == max_y || grid[x][y+1] == '|' {
			grid[x][y] = '|';
			continue;
		}

		let mut fall_right = false;
		let mut fall_left = false;
		let mut check_x_left = x;
		let mut check_x_right = x;
		
		while grid[check_x_right][y+1] != '.' && grid[check_x_right+1][y] != '#' {
			check_x_right += 1;
		}
		if grid[check_x_right-1][y] == '|' && 
		   grid[check_x_right-1][y+1] == '|' {
			   continue;
		   }
		if grid[check_x_right][y+1] == '.' {
			fall_right = true;
		}
		
		while grid[check_x_left][y+1] != '.' && grid[check_x_left-1][y] != '#' {
			check_x_left -= 1;
		}
		if grid[check_x_left+1][y] == '|' && 
		grid[check_x_left+1][y+1] == '|' {
			continue;
		}
		if grid[check_x_left-1][y] != '#' {
			fall_left = true;
		} 
		if !fall_left && !fall_right {
			for i in check_x_left..=check_x_right {
				grid[i][y] = '~';
			}
			sources.insert((x,y-1));
		} else {
			for i in check_x_left..=check_x_right {
				grid[i][y] = '|';
				if sources.contains(&(i, y)) {
					sources.remove(&(i, y));
				}
			}
			if fall_left {
				sources.insert((check_x_left, y));
			}
			if fall_right {
				sources.insert((check_x_right, y));
			}
		}
	}

	let mut total = 0;
	let mut resting = 0;
	for x in 0..2000 {
		for y in min_y..=max_y {
			if grid[x][y] == '~' || grid[x][y] == '|' {
				total += 1;
			}
			if grid[x][y] == '~' {
				resting += 1;
			}
		}
	}

	(total, resting)
}

pub fn main() {
	let grid = parse_input("./input/day17/input.txt");

	let p1_timer = Instant::now();
    let (p1_result, p2_result) = part1(&grid);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 2: {}", p2_result);
	println!("Part 1+2 Time: {:?}", p1_time);
}