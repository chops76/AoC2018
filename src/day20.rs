use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Dirs {
	north: bool,
	south: bool,
	east: bool,
	west: bool
}

fn parse_input(path: &str) -> Vec<char> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.chars().collect()
}

fn create_graph(param_data: &Vec<char>) -> HashMap<(i32, i32), Dirs> {
	let mut things = VecDeque::new();
	let mut visited = HashSet::new();
	let mut graph = HashMap::new();
	graph.insert((0, 0), Dirs {north: false, south: false, east: false, west: false});
	things.push_back(((0, 0), 1));
	while things.len() > 0 {
		let (coord, pos) = things.pop_front().unwrap();
		if visited.contains(&(coord, pos)) {
			continue;
		}
		visited.insert((coord, pos));
		
		match param_data[pos] {
			'N' => { 
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.north = true;
				let mut coord = coord;
				coord.1 -= 1;
				if !graph.contains_key(&coord) {
					graph.insert(coord, Dirs {north: false, south: false, east: false, west: false});
				}
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.south = true;
				things.push_back((coord, pos + 1)); 
			},
			'E' => { 
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.east = true;
				let mut coord = coord;
				coord.0 += 1;
				if !graph.contains_key(&coord) {
					graph.insert(coord, Dirs {north: false, south: false, east: false, west: false});
				}
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.west = true;
				things.push_back((coord, pos + 1)); 
			},
			'W' => { 
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.west = true;
				let mut coord = coord;
				coord.0 -= 1;
				if !graph.contains_key(&coord) {
					graph.insert(coord, Dirs {north: false, south: false, east: false, west: false});
				}
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.east = true;
				things.push_back((coord, pos + 1)); 
			},
			'S' => { 
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.south = true;
				let mut coord = coord;
				coord.1 += 1;
				if !graph.contains_key(&coord) {
					graph.insert(coord, Dirs {north: false, south: false, east: false, west: false});
				}
				let mut dirs = graph.get_mut(&coord).unwrap();
				dirs.north = true;
				things.push_back((coord, pos + 1)); },
			'(' => {
				let mut depth = 0;
				things.push_back((coord, pos + 1));
				let mut pos = pos + 1;
				while !(depth == 0 && param_data[pos] == ')') {
					if param_data[pos] == '|' && depth == 0 {
						things.push_back((coord, pos + 1));
					}
					if param_data[pos] == '(' {
						depth += 1;
					} 
					if param_data[pos] == ')' {
						depth -= 1;
					}
					pos += 1;
				}
			},
			'|' => {
				let mut depth = 0;
				let mut pos = pos + 1;
				while !(depth == 0 && param_data[pos] == ')') {
					if param_data[pos] == '(' {
						depth += 1;
					} 
					if param_data[pos] == ')' {
						depth -= 1;
					}	
					pos += 1;				
				}
				things.push_back((coord, pos + 1));
			},
			')' => {
				things.push_back((coord, pos + 1));
			}
			_ => {}
		}
	}

	graph
}

fn part1(param_data: &Vec<char>) -> usize {	
	let graph = create_graph(param_data);
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_front((0, (0, 0)));
	while queue.len() > 0 {
		let (count, node) = queue.pop_front().unwrap();
		if visited.contains(&node) {
			continue;
		}
		visited.insert(node);
		if visited.len() == graph.len() {
			return count;
		}
		if graph[&node].north {
			let mut new_node = node.clone();
			new_node.1 -= 1;
			queue.push_back((count+1, new_node));
		}
		if graph[&node].south {
			let mut new_node = node.clone();
			new_node.1 += 1;
			queue.push_back((count+1, new_node));
		}
		if graph[&node].east {
			let mut new_node = node.clone();
			new_node.0 += 1;
			queue.push_back((count+1, new_node));
		}
		if graph[&node].west {
			let mut new_node = node.clone();
			new_node.0 -= 1;
			queue.push_back((count+1, new_node));
		}
	}
	println!("Ran out of nodes!");
	0
}

fn part2(param_data: &Vec<char>) -> usize {	
	let graph = create_graph(param_data);
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	let mut total = 0;
	queue.push_front((0, (0, 0)));
	while queue.len() > 0 {
		let (count, node) = queue.pop_front().unwrap();
		if visited.contains(&node) {
			continue;
		}
		visited.insert(node);
		if count >= 1000 {
			total += 1;
		}
		if visited.len() == graph.len() {
			return total;
		}
		if graph[&node].north {
			let mut new_node = node.clone();
			new_node.1 -= 1;
			queue.push_back((count+1, new_node));
		}
		if graph[&node].south {
			let mut new_node = node.clone();
			new_node.1 += 1;
			queue.push_back((count+1, new_node));
		}
		if graph[&node].east {
			let mut new_node = node.clone();
			new_node.0 += 1;
			queue.push_back((count+1, new_node));
		}
		if graph[&node].west {
			let mut new_node = node.clone();
			new_node.0 -= 1;
			queue.push_back((count+1, new_node));
		}
	}
	println!("Ran out of nodes!");
	0
}

pub fn main() {
    let data = parse_input("./input/day20/input.txt");

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