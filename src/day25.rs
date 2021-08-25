use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;

type Point = (i64, i64, i64,i64);
type Input = Vec<Point>;

fn parse_line(s: &str) -> Point {
	let spl = s.split(",").collect::<Vec<&str>>();

	(spl[0].parse().unwrap(), spl[1].parse().unwrap(), spl[2].parse().unwrap(), spl[3].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn close(p1: &Point, p2: &Point) -> bool {
	(p1.0-p2.0).abs() + (p1.1-p2.1).abs() + (p1.2-p2.2).abs() + (p1.3-p2.3).abs() <= 3
}

fn part1(points: &Input) -> usize {
	let mut cons = points.iter().enumerate().collect::<Vec<(usize, &Point)>>();
	for i in 0..cons.len() - 1 {
		for j in i..cons.len() {
			if cons[i].0 == cons[j].0 {
				continue;
			}
			if close(cons[i].1, cons[j].1) {
				let old = cons[j].0;
				for k in 0..cons.len() {
					if cons[k].0 == old {
						cons[k] = (cons[i].0, cons[k].1);
					}
				}
			}
		}
	}

	cons.iter().map(|(n,_)| *n).collect::<HashSet<usize>>().len()
}

pub fn main() {
	let points = parse_input("./input/day25/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&points);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time); 
}