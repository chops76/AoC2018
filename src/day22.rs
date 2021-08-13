use std::time::Instant;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
	tool: Tool
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Tool {
	GEAR,
	TORCH,
	NEITHER
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(depth: u64, x_size: usize, y_size: usize) -> u64 {	
	let mut graph = vec![vec![0; y_size+1];x_size+1];
	for i in 1..=x_size {
		graph[i][0] = (16807 * i as u64 + depth) % 20183;
	}
	for i in 1..=y_size {
		graph[0][i] = (48271 * i as u64 + depth) % 20183;
	}
	for x in 1..=x_size {
		for y in 1..=y_size {
			graph[x][y] = (graph[x-1][y] * graph[x][y-1] + depth) % 20183;
		}
	}
	graph[x_size][y_size] = 0;

	let mut total = 0;
	for x in 0..=x_size {
		for y in 0..=y_size {
			total += graph[x][y] % 3;
		}
	}

	total
}

fn part2(depth: u64, x_size: usize, y_size: usize) -> usize {	
	let mut graph = vec![vec![0; y_size*8+1];x_size*8+1];
	for i in 1..=x_size*8 {
		graph[i][0] = (16807 * i as u64 + depth) % 20183;
	}
	for i in 1..=y_size*8 {
		graph[0][i] = (48271 * i as u64 + depth) % 20183;
	}
	for x in 1..=x_size*8 {
		for y in 1..=y_size*8 {
			graph[x][y] = (graph[x-1][y] * graph[x][y-1] + depth) % 20183;
		}
	}
	graph[x_size][y_size] = 0;

	let mut heap = BinaryHeap::new();
	let mut visited = HashSet::new();

    heap.push(State { tool: Tool::TORCH, cost: 0, position: (0,0) });
	while heap.len() > 0 {
		let cur_state = heap.pop().unwrap();
		if visited.contains(&(cur_state.tool, cur_state.position)) {
			continue;
		}
		if cur_state.position.0 == x_size &&
		   cur_state.position.1 == y_size && 
		   cur_state.tool == Tool::TORCH {
			   println!("Found it!");
			   return cur_state.cost;
		   }
		//println!("Working state: {:?}", cur_state);
		visited.insert((cur_state.tool, cur_state.position));
		let cur_terrain = graph[cur_state.position.0][cur_state.position.1] % 3;
		let new_tool = match cur_terrain {
			0 => if cur_state.tool == Tool::TORCH { Tool::GEAR } else { Tool::TORCH },
			1 => if cur_state.tool == Tool::GEAR { Tool::NEITHER } else { Tool::GEAR },
			2 => if cur_state.tool == Tool::TORCH { Tool::NEITHER } else { Tool::TORCH },
			_ => { println!("ERROR: Invalid Tool"); Tool::NEITHER }
		};
		heap.push(State { tool: new_tool, cost: cur_state.cost + 7, position: cur_state.position } );

		if cur_state.position.1 > 0 {
			let new_pos = (cur_state.position.0, cur_state.position.1 - 1);
			let new_terrain = graph[new_pos.0][new_pos.1] % 3;
			if !(new_terrain == 0 && cur_state.tool == Tool::NEITHER) &&
			   !(new_terrain == 1 && cur_state.tool == Tool::TORCH) && 
			   !(new_terrain == 2 && cur_state.tool == Tool::GEAR) {
				heap.push(State { tool: cur_state.tool, cost: cur_state.cost + 1, position: new_pos });
			}
		}

		if cur_state.position.0 > 0 {
			let new_pos = (cur_state.position.0 - 1, cur_state.position.1);
			let new_terrain = graph[new_pos.0][new_pos.1] % 3;
			if !(new_terrain == 0 && cur_state.tool == Tool::NEITHER) &&
			   !(new_terrain == 1 && cur_state.tool == Tool::TORCH) && 
			   !(new_terrain == 2 && cur_state.tool == Tool::GEAR) {
				heap.push(State { tool: cur_state.tool, cost: cur_state.cost + 1, position: new_pos });
			}
		}

		if cur_state.position.1 < y_size * 8 {
			let new_pos = (cur_state.position.0, cur_state.position.1 + 1);
			let new_terrain = graph[new_pos.0][new_pos.1] % 3;
			if !(new_terrain == 0 && cur_state.tool == Tool::NEITHER) &&
			   !(new_terrain == 1 && cur_state.tool == Tool::TORCH) && 
			   !(new_terrain == 2 && cur_state.tool == Tool::GEAR) {
				heap.push(State { tool: cur_state.tool, cost: cur_state.cost + 1, position: new_pos });
			}
		}

		if cur_state.position.0 < x_size * 8 {
			let new_pos = (cur_state.position.0 + 1, cur_state.position.1);
			let new_terrain = graph[new_pos.0][new_pos.1] % 3;
			if !(new_terrain == 0 && cur_state.tool == Tool::NEITHER) &&
			   !(new_terrain == 1 && cur_state.tool == Tool::TORCH) && 
			   !(new_terrain == 2 && cur_state.tool == Tool::GEAR) {
				heap.push(State { tool: cur_state.tool, cost: cur_state.cost + 1, position: new_pos });
			}
		}
	}

	println!("ERROR: Out of nodes");
	0
}

pub fn main() {
	let p1_timer = Instant::now();
	let p1_result = part1(8787, 10, 725);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(8787,10,725);
	//let p2_result = part2(510,10,10);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);  
}