use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

type Bot = ((i64, i64, i64), i64);
type Input = Vec<Bot>;

fn parse_line(s: &str) -> ((i64, i64, i64), i64) {
	let spl = s.split(",").collect::<Vec<&str>>();

	((spl[0][5..].parse().unwrap(), spl[1].parse().unwrap(), spl[2][..spl[2].len()-1].parse().unwrap()), spl[3][3..].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(bots: &Input) -> usize {
	let largest_bot = bots.iter().max_by_key(|(_, rad)| rad).unwrap();
	bots.iter().filter(|b| (b.0.0 - largest_bot.0.0).abs() +
	                       (b.0.1 - largest_bot.0.1).abs() +
						   (b.0.2 - largest_bot.0.2).abs() <= largest_bot.1).count()
}

fn within_range(bot1: &((i64, i64, i64), i64), bot2: &((i64, i64, i64), i64)) -> bool {
	(bot1.0.0 - bot2.0.0).abs() + (bot1.0.1 - bot2.0.1).abs() + (bot1.0.2 - bot2.0.2).abs() <= bot1.1 + bot2.1
}

fn find_clique (p: HashSet<Bot>, r: HashSet<Bot>, x: HashSet<Bot>, best: &mut HashSet<Bot>, neighbors: &HashMap<Bot, HashSet<Bot>>) {
	if p.is_empty() && x.is_empty() {
		// Need to study how Rust wants me to do this, because bleah
		if r.len() > best.len() {
			best.clear();
			for v in r {
				best.insert(v);
			}
		}
	} else {
		let max_neighbors_p_and_x = p.union(&x).max_by_key(|b| neighbors[b].len()).unwrap();
		let p_no_neighbors = p.difference(&neighbors[max_neighbors_p_and_x]);
		for v in p_no_neighbors {
			let v_neighbors = neighbors[v].clone();
			let mut rpv = r.clone();
			rpv.insert(*v);
			find_clique(
				p.intersection(&v_neighbors).map(|b| *b).collect(),
				rpv,
				x.intersection(&v_neighbors).map(|b| *b).collect(),
				best, neighbors);
		}
	}
}

fn part2(bots: &Input) -> usize {
	let mut neighbors = HashMap::new();
	for bot in bots {
		let mut hs = HashSet::new();
		for other in bots {
			if bot != other && within_range(bot, other) {
				hs.insert(*other);
			}
		}
		neighbors.insert(*bot, hs);
	}

	let mut best_set = HashSet::new();
	let r = HashSet::new();
	let x = HashSet::new();
	find_clique(neighbors.keys().map(|i| i.clone()).collect(), r, x, &mut best_set, &neighbors);
	let ans = best_set.iter().map(|i| i.0.0.abs() + i.0.1.abs() + i.0.2.abs() - i.1).max().unwrap();
	ans as usize
}

pub fn main() {
	let bots = parse_input("./input/day23/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&bots);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time); 

	let p2_timer = Instant::now();
    let p2_result = part2(&bots);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);  
}