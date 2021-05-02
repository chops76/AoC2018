use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;
use std::cmp;

type Input = Vec<(usize, usize)>;

fn parse_line(s: &str) -> (usize, usize) {
	let spl = s.split(", ").collect::<Vec<&str>>();
	(spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(pts: &Input) -> usize {
	let mut grid = vec![vec![-1;2000]; 2000];
	let mut min_x = 1000 + pts[0].0;
	let mut max_x = 1000 + pts[0].0;
	let mut min_y = 1000 + pts[0].1;
	let mut max_y = 1000 + pts[0].1;
	for i in 0..pts.len() {
		grid[pts[i].0+1000][pts[i].1+1000] = i as i32;
		min_x = cmp::min(min_x, pts[i].0 + 1000);
		max_x = cmp::max(max_x, pts[i].0 + 1000);
		min_y = cmp::min(min_y, pts[i].1 + 1000);
		max_y = cmp::max(max_y, pts[i].1 + 1000);
	}

	for _ in 0..100 { 
		let mut new_grid = grid.clone();
		for x in min_x-1 ..= max_x+1 {
			for y in min_y-1 ..= max_y+1 {
				if grid[x][y] != -1 {
					continue;
				}
				let mut val = -1;
				for (x1,y1) in [(0, 1), (2, 1), (1, 0), (1, 2)].iter() {
					let tmp = grid[x+x1-1][y+y1-1];
					if tmp != -1 {
						if val == -1 {
							val = tmp;
						} else if val != tmp {
							val = -2;
						}
					}
				}
				new_grid[x][y] = val;
			}
		}

		min_x -= 1;
		max_x += 1;
		min_y -= 1;
		max_y += 1;
		grid = new_grid;
	}
	let mut hs = HashSet::new();

	let mut counts = vec![0;pts.len()];
	for x in min_x-1 ..= max_x+1 {
		for y in min_y-1 ..= max_y+1 {
			if grid[x][y] >= 0 {
				counts[grid[x][y] as usize] += 1;
				continue;
			}
		}
	}

	for x in min_x-1 ..= max_x+1 {
		for y in min_y-1 ..= max_y+1 {
			if grid[x][y] != -1 {
				continue;
			}
			for (x1,y1) in [(0, 1), (2, 1), (1, 0), (1, 2)].iter() {
				let tmp = grid[x+x1-1][y+y1-1];
				if grid[x+x1-1][y+y1-1] >= 0 {
					hs.insert(tmp);
				}
			}
		}
	}

	let mut max = 0;
	for (number, val) in counts.iter().enumerate() {
		if hs.contains(&(number as i32)) {
			continue;
		}
		if val > &max {
			max = *val;
		}
	}

	max
}

fn part2(pts: &Input) -> usize {
	let mut grid = vec![vec![-1;2000]; 2000];
	let mut min_x = 1000 + pts[0].0;
	let mut max_x = 1000 + pts[0].0;
	let mut min_y = 1000 + pts[0].1;
	let mut max_y = 1000 + pts[0].1;
	for i in 0..pts.len() {
		grid[pts[i].0+1000][pts[i].1+1000] = i as i32;
		min_x = cmp::min(min_x, pts[i].0 + 1000);
		max_x = cmp::max(max_x, pts[i].0 + 1000);
		min_y = cmp::min(min_y, pts[i].1 + 1000);
		max_y = cmp::max(max_y, pts[i].1 + 1000);
	}

	let mut count = 0;
	for x in max_x as i32-10000..min_x as i32+10000 {
		for y in max_y as i32-10000..min_y as i32+10000 {
			let mut sum = 0;
			for p in pts {
				sum += ((x as i32) - (p.0 as i32)).abs();
				sum += ((y as i32) - (p.1 as i32)).abs();
			}
			if sum < 10000 {
				count += 1;
			}
		}
	}
	count
}

pub fn main() {
    let points = parse_input("./input/day6/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&points);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&points);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}