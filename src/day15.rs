use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp;

#[derive(Debug)]
#[derive(Clone)]
struct Unit {
	x: usize,
	y: usize,
	hp: i32,
	elf: bool,
	dead: bool
}

type Input = Vec<Vec<char>>;

fn parse_line(s: &str) -> Vec<char> {
	s.chars().collect()
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn find_targets(grid: &Input, enemy: char) -> HashSet<(usize, usize)> {
	let mut targets = HashSet::new();
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] == enemy {
				if grid[y-1][x] == '.' {
					targets.insert((x, y-1));
				}
				if grid[y+1][x] == '.' {
					targets.insert((x, y+1));
				}
				if grid[y][x-1] == '.' {
					targets.insert((x-1, y));
				}
				if grid[y][x+1] == '.' {
					targets.insert((x+1, y));
				}
			}
		}
	}

	targets
}

fn calc_paths(cur_path: &mut Vec<(usize, usize)>, distances: &HashSet<(usize, usize, usize)>, dist: usize, 
              paths: &mut Vec<Vec<(usize, usize)>>) {
	if dist == 0 {
		let mut rev_path = cur_path.clone();
		rev_path.reverse();
		paths.push(rev_path);
		return;
	}
	let cur_x = cur_path[cur_path.len()-1].0;
	let cur_y = cur_path[cur_path.len()-1].1;
	if distances.contains(&(cur_x-1, cur_y, dist)) {
		cur_path.push((cur_x-1, cur_y));
		calc_paths(cur_path, distances, dist - 1, paths);
		cur_path.pop();
	}
	if distances.contains(&(cur_x+1, cur_y, dist)) {
		cur_path.push((cur_x+1, cur_y));
		calc_paths(cur_path, distances, dist - 1, paths);
		cur_path.pop();
	}
	if distances.contains(&(cur_x, cur_y-1, dist)) {
		cur_path.push((cur_x, cur_y-1));
		calc_paths(cur_path, distances, dist - 1, paths);
		cur_path.pop();
	}
	if distances.contains(&(cur_x, cur_y+1, dist)) {
		cur_path.push((cur_x, cur_y+1));
		calc_paths(cur_path, distances, dist - 1, paths);
		cur_path.pop();
	}
}

fn calc_move(grid: &Input, start_x: usize, start_y: usize, targets: HashSet<(usize, usize)>) -> (usize, usize) {
	let mut visited = HashSet::new();
	let mut distances = HashSet::new();
	let mut queue = VecDeque::new();
	let mut good_ones = Vec::new();
	let mut found_dist = 0;
	queue.push_back((start_x, start_y, 0));
	while queue.len() != 0 {
		let (x, y, dist) = queue.pop_front().unwrap();
		if visited.contains(&(x, y)) {
			continue;
		}
		if found_dist != 0 && dist != found_dist {
			break;
		}
		visited.insert((x, y));
		distances.insert((x, y, dist));
		if targets.contains(&(x, y)) {
			good_ones.push((x, y));
			found_dist = dist;
		}
		if grid[y-1][x] == '.' && !visited.contains(&(x, y-1)) {
			queue.push_back((x, y-1, dist+1));
		}
		if grid[y+1][x] == '.' && !visited.contains(&(x, y+1)) {
			queue.push_back((x, y+1, dist+1));
		}
		if grid[y][x-1] == '.' && !visited.contains(&(x-1, y)) {
			queue.push_back((x-1, y, dist+1));
		}
		if grid[y][x+1] == '.' && !visited.contains(&(x+1, y)) {
			queue.push_back((x+1, y, dist+1));
		}
	}
	
	if good_ones.len() == 0 {
		return (0,0);
	}
	let mut paths = Vec::new();
	good_ones.sort_by(|a,b| if a.1 == b.1 { a.0.cmp(&b.0) } else { a.1.cmp(&b.1) });
	let mut tmp = Vec::new();
	tmp.push((good_ones[0].0, good_ones[0].1));
	calc_paths(&mut tmp, &distances, found_dist - 1, &mut paths);
	let mut first_moves = Vec::new();
	for path in paths {
		first_moves.push(path[0]);
	}
	first_moves.sort_by(|a,b| if a.1 == b.1 { a.0.cmp(&b.0) } else { a.1.cmp(&b.1) });
	return first_moves[0];
}

fn engaged(dude: &Unit, grid: &Input) -> bool {
	let enemy = if dude.elf { 'G' } else { 'E'};

	grid[dude.y-1][dude.x] == enemy || grid[dude.y+1][dude.x] == enemy ||
	    grid[dude.y][dude.x-1] == enemy || grid[dude.y][dude.x+1] == enemy
}

fn num_hp(x: usize, y: usize, dudes: &Vec<Unit>) -> i32 {
	for i in 0..dudes.len() {
		if dudes[i].x == x && dudes[i].y == y && !dudes[i].dead {
			return dudes[i].hp;
		}
	}
	println!("ERROR: Couldn't find HP");
	0
}

fn who_to_attack(dude: &Unit, grid: &Input, dudes: &Vec<Unit>) -> (usize, usize) {
	let enemy = if dude.elf { 'G' } else { 'E'};

	let mut lowest = 500;
	let mut low_x = 0;
	let mut low_y = 0;

	if grid[dude.y-1][dude.x] == enemy {
		low_x = dude.x;
		low_y = dude.y-1;
		lowest = num_hp(low_x, low_y, dudes);
	}
	if grid[dude.y][dude.x-1] == enemy {
		let tmp = num_hp(dude.x-1, dude.y, dudes);
		if tmp < lowest {
			low_x = dude.x-1;
			low_y = dude.y;
			lowest = tmp;
		}
	}
	if grid[dude.y][dude.x+1] == enemy {
		let tmp = num_hp(dude.x+1, dude.y, dudes);
		if tmp < lowest {
			low_x = dude.x+1;
			low_y = dude.y;
			lowest = tmp;
		}
	}
	if grid[dude.y+1][dude.x] == enemy {
		let tmp = num_hp(dude.x, dude.y+1, dudes);
		if tmp < lowest {
			low_x = dude.x;
			low_y = dude.y+1;
		}
	}
	if low_x == 0 && low_y == 0 {
		println!("Warning: Couldn't find lowest!");
	}
	(low_x, low_y)
}

fn part1(grid: &Input) -> usize {
	let mut dudes = Vec::new();
	let mut grid = grid.clone();

	let mut num_elves = 0;
	let mut num_gobs = 0;

	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			if grid[y][x] == 'E' {
				dudes.push(Unit { x: x, y: y, hp: 200, elf: true, dead: false });
				num_elves += 1;
			} else if grid[y][x] == 'G' {
				dudes.push(Unit { x: x, y: y, hp: 200, elf: false, dead: false });
				num_gobs += 1;
			}
		}
	}

	for round in 0..500 {
		dudes.sort_by(|a,b| if a.y == b.y { a.x.cmp(&b.x) } else { a.y.cmp(&b.y) });
		for i in 0..dudes.len() {
			if dudes[i].dead {
				continue;
			}

			if (dudes[i].elf == false && num_elves == 0 ) || 
			   (dudes[i].elf && num_gobs == 0) {
				let mut sum = 0;
				for k in 0..dudes.len() {
					if !dudes[k].dead {
						sum += dudes[k].hp;
					}
				}
			
				return round * sum as usize;
			}

			if !engaged(&dudes[i], &grid) {
				let enemy = if dudes[i].elf { 'G' } else { 'E'};
				let targets = find_targets(&grid, enemy);
				if targets.len() != 0 {
					let new_pos = calc_move(&grid, dudes[i].x, dudes[i].y, targets);
					if new_pos != (0,0) {
						grid[new_pos.1][new_pos.0] = grid[dudes[i].y][dudes[i].x];
						grid[dudes[i].y][dudes[i].x] = '.';
						dudes[i].x = new_pos.0;
						dudes[i].y = new_pos.1;
					}
				} else {

				}
			}
			if engaged(&dudes[i], &grid) {
				let attack_pos = who_to_attack(&dudes[i], &grid, &dudes);
				for j in 0..dudes.len() {
					if grid[attack_pos.1][attack_pos.0] == '.' {
						println!("ATTACKING EMPTY SPACE");
					}
					if dudes[j].x == attack_pos.0 && dudes[j].y == attack_pos.1 {
						dudes[j].hp -= 3;
						if dudes[j].hp <= 0 {
							dudes[j].dead = true;
							grid[dudes[j].y][dudes[j].x] = '.';
							if dudes[j].elf {
								num_elves -= 1; 
							} else {
								num_gobs -= 1;
							}

						}
						break;
					}
				}
			}
		}
		dudes = dudes.iter().filter(|d| !d.dead).map(|d| d.clone()).collect();
	}
	0
}

fn part2(grid: &Input) -> usize {
	let mut dudes = Vec::new();
	let mut grid = grid.clone();

	let mut num_elves = 0;
	let mut num_gobs = 0;

	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			if grid[y][x] == 'E' {
				dudes.push(Unit { x: x, y: y, hp: 200, elf: true, dead: false });
				num_elves += 1;
			} else if grid[y][x] == 'G' {
				dudes.push(Unit { x: x, y: y, hp: 200, elf: false, dead: false });
				num_gobs += 1;
			}
		}
	}

	for round in 0..500 {
		dudes.sort_by(|a,b| if a.y == b.y { a.x.cmp(&b.x) } else { a.y.cmp(&b.y) });
		for i in 0..dudes.len() {
			if dudes[i].dead {
				continue;
			}

			if (dudes[i].elf == false && num_elves == 0 ) || 
			   (dudes[i].elf && num_gobs == 0) {
				let mut sum = 0;
				for k in 0..dudes.len() {
					if !dudes[k].dead {
						sum += dudes[k].hp;
					}
				}
				
				return round * sum as usize;
			}

			if !engaged(&dudes[i], &grid) {
				let enemy = if dudes[i].elf { 'G' } else { 'E'};
				let targets = find_targets(&grid, enemy);
				if targets.len() != 0 {
					let new_pos = calc_move(&grid, dudes[i].x, dudes[i].y, targets);
					if new_pos != (0,0) {
						grid[new_pos.1][new_pos.0] = grid[dudes[i].y][dudes[i].x];
						grid[dudes[i].y][dudes[i].x] = '.';
						dudes[i].x = new_pos.0;
						dudes[i].y = new_pos.1;
					}
				} else {

				}
			}
			if engaged(&dudes[i], &grid) {
				let attack_pos = who_to_attack(&dudes[i], &grid, &dudes);
				for j in 0..dudes.len() {
					if grid[attack_pos.1][attack_pos.0] == '.' {
						println!("ATTACKING EMPTY SPACE");
					}
					if dudes[j].x == attack_pos.0 && dudes[j].y == attack_pos.1 {
						if dudes[j].elf {
							dudes[j].hp -= 3;
						} else {
							dudes[j].hp -= 25;
						}
						
						if dudes[j].hp <= 0 {
							dudes[j].dead = true;
							grid[dudes[j].y][dudes[j].x] = '.';
							if dudes[j].elf {
								println!("ELF DIED");
								return 0;
							} else {
								num_gobs -= 1;
							}

						}
						break;
					}
				}
			}
		}
		dudes = dudes.iter().filter(|d| !d.dead).map(|d| d.clone()).collect();
	}
	0
}

pub fn main() {
	let grid = parse_input("./input/day15/input.txt");

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