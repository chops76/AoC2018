use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum Direction {
	DirUp,
	DirRight,
	DirDown,
	DirLeft
}

#[derive(Debug)]
#[derive(Clone)]
enum NextDir {
	NextDirLeft,
	NextDirStraight,
	NextDirRight
}

#[derive(Debug)]
enum GraphItemType {
	Cross,
	Curve,
	BackCurve
}

#[derive(Debug)]
#[derive(Clone)]
struct Cart {
	x: usize,
	y: usize,
	dir: Direction,
	next_dir: NextDir, 
	crashed: bool
}

type Input = (HashMap<(usize,usize),GraphItemType>,Vec<Cart>);

fn parse_input(path: &str) -> Input {
	let mut intersections = HashMap::new();
	let mut carts = Vec::new();

	let f = File::open(path).unwrap();
	let lines = BufReader::new(f).lines().flatten();

	for (y,line) in lines.enumerate() {
		for (x,c) in line.chars().enumerate() {
			match c {
				'+' => { intersections.insert((x,y),GraphItemType::Cross); },
				'/' => { intersections.insert((x,y),GraphItemType::Curve); },
				'\\' => { intersections.insert((x,y),GraphItemType::BackCurve); },
				'^' => { carts.push( Cart { x: x, y: y, dir: Direction::DirUp, next_dir: NextDir::NextDirLeft, crashed: false } ); },
				'v' => { carts.push( Cart { x: x, y: y, dir: Direction::DirDown, next_dir: NextDir::NextDirLeft, crashed: false } ); },
				'<' => { carts.push( Cart { x: x, y: y, dir: Direction::DirLeft, next_dir: NextDir::NextDirLeft, crashed: false } ); },
				'>' => { carts.push( Cart { x: x, y: y, dir: Direction::DirRight, next_dir: NextDir::NextDirLeft, crashed: false } ); },
				_ => {}
			} 
		}
	}

	(intersections,carts)
}

fn part1(intersections: &HashMap<(usize,usize),GraphItemType>, carts: &Vec<Cart>) -> (usize,usize) {
	let mut sorted_carts = carts.clone();
	loop {
		sorted_carts.sort_by(|a,b| if a.y == b.y { a.x.cmp(&b.x) } else { a.y.cmp(&b.y) });
		for i in 0..sorted_carts.len() {
			match sorted_carts[i].dir {
				Direction::DirUp => sorted_carts[i].y -= 1,
				Direction::DirLeft => sorted_carts[i].x -= 1,
				Direction::DirDown => sorted_carts[i].y += 1,
				Direction::DirRight => sorted_carts[i].x += 1
			}
			let others = sorted_carts.iter().enumerate().filter(|(a,_)|*a != i).map(|(_,c)|(c.x,c.y)).collect::<Vec<(usize,usize)>>();
			if others.contains(&(sorted_carts[i].x,sorted_carts[i].y)) {
				return (sorted_carts[i].x,sorted_carts[i].y);
			}
			if intersections.contains_key(&(sorted_carts[i].x, sorted_carts[i].y)) {
				match intersections[&(sorted_carts[i].x, sorted_carts[i].y)] {
					GraphItemType::Cross => {
						match sorted_carts[i].next_dir {
							NextDir::NextDirLeft => {
								match sorted_carts[i].dir {
									Direction::DirUp => sorted_carts[i].dir = Direction::DirLeft,
									Direction::DirLeft => sorted_carts[i].dir = Direction::DirDown,
									Direction::DirDown => sorted_carts[i].dir = Direction::DirRight,
									Direction::DirRight => sorted_carts[i].dir = Direction::DirUp
								};
								sorted_carts[i].next_dir = NextDir::NextDirStraight
							},
							NextDir::NextDirRight => {
								match sorted_carts[i].dir {
									Direction::DirUp => sorted_carts[i].dir = Direction::DirRight,
									Direction::DirLeft => sorted_carts[i].dir = Direction::DirUp,
									Direction::DirDown => sorted_carts[i].dir = Direction::DirLeft,
									Direction::DirRight => sorted_carts[i].dir = Direction::DirDown
								};
								sorted_carts[i].next_dir = NextDir::NextDirLeft
							},
							NextDir::NextDirStraight => {
								sorted_carts[i].next_dir = NextDir::NextDirRight
							}
						}
					},
					GraphItemType::Curve => {
						match sorted_carts[i].dir {
							Direction::DirUp => sorted_carts[i].dir = Direction::DirRight,
							Direction::DirLeft => sorted_carts[i].dir = Direction::DirDown,
							Direction::DirDown => sorted_carts[i].dir = Direction::DirLeft,
							Direction::DirRight => sorted_carts[i].dir = Direction::DirUp
						};					
					},
					GraphItemType::BackCurve => {
						match sorted_carts[i].dir {
							Direction::DirUp => sorted_carts[i].dir = Direction::DirLeft,
							Direction::DirLeft => sorted_carts[i].dir = Direction::DirUp,
							Direction::DirDown => sorted_carts[i].dir = Direction::DirRight,
							Direction::DirRight => sorted_carts[i].dir = Direction::DirDown
						};					
					}
				}
			}
		}
	}
}

fn part2(intersections: &HashMap<(usize,usize),GraphItemType>, carts: &Vec<Cart>) -> (usize,usize) {
	let mut sorted_carts = carts.clone();
	loop {
		sorted_carts.sort_by(|a,b| if a.y == b.y { a.x.cmp(&b.x) } else { a.y.cmp(&b.y) });
		for i in 0..sorted_carts.len() {
			if sorted_carts[i].crashed == true {
				continue;
			}
			match sorted_carts[i].dir {
				Direction::DirUp => sorted_carts[i].y -= 1,
				Direction::DirLeft => sorted_carts[i].x -= 1,
				Direction::DirDown => sorted_carts[i].y += 1,
				Direction::DirRight => sorted_carts[i].x += 1
			}
			let others = sorted_carts.iter().enumerate().filter(|(a,_)|*a != i).map(|(_,c)|(c.x,c.y)).collect::<Vec<(usize,usize)>>();
			if others.contains(&(sorted_carts[i].x,sorted_carts[i].y)) {
				for j in 0..sorted_carts.len() {
					if sorted_carts[j].x == sorted_carts[i].x && sorted_carts[j].y == sorted_carts[j].y {
						sorted_carts[j].crashed = true;
					}
				}
			}
			if intersections.contains_key(&(sorted_carts[i].x, sorted_carts[i].y)) {
				match intersections[&(sorted_carts[i].x, sorted_carts[i].y)] {
					GraphItemType::Cross => {
						match sorted_carts[i].next_dir {
							NextDir::NextDirLeft => {
								match sorted_carts[i].dir {
									Direction::DirUp => sorted_carts[i].dir = Direction::DirLeft,
									Direction::DirLeft => sorted_carts[i].dir = Direction::DirDown,
									Direction::DirDown => sorted_carts[i].dir = Direction::DirRight,
									Direction::DirRight => sorted_carts[i].dir = Direction::DirUp
								};
								sorted_carts[i].next_dir = NextDir::NextDirStraight
							},
							NextDir::NextDirRight => {
								match sorted_carts[i].dir {
									Direction::DirUp => sorted_carts[i].dir = Direction::DirRight,
									Direction::DirLeft => sorted_carts[i].dir = Direction::DirUp,
									Direction::DirDown => sorted_carts[i].dir = Direction::DirLeft,
									Direction::DirRight => sorted_carts[i].dir = Direction::DirDown
								};
								sorted_carts[i].next_dir = NextDir::NextDirLeft
							},
							NextDir::NextDirStraight => {
								sorted_carts[i].next_dir = NextDir::NextDirRight
							}
						}
					},
					GraphItemType::Curve => {
						match sorted_carts[i].dir {
							Direction::DirUp => sorted_carts[i].dir = Direction::DirRight,
							Direction::DirLeft => sorted_carts[i].dir = Direction::DirDown,
							Direction::DirDown => sorted_carts[i].dir = Direction::DirLeft,
							Direction::DirRight => sorted_carts[i].dir = Direction::DirUp
						};					
					},
					GraphItemType::BackCurve => {
						match sorted_carts[i].dir {
							Direction::DirUp => sorted_carts[i].dir = Direction::DirLeft,
							Direction::DirLeft => sorted_carts[i].dir = Direction::DirUp,
							Direction::DirDown => sorted_carts[i].dir = Direction::DirRight,
							Direction::DirRight => sorted_carts[i].dir = Direction::DirDown
						};					
					}
				}
			}
		}
		sorted_carts = sorted_carts.iter().filter(|c| !c.crashed ).map(|c| c.clone()).collect();
		if sorted_carts.len() == 1 {
			return (sorted_carts[0].x, sorted_carts[0].y);
		}
	}
}

pub fn main() {
	let (intersections,carts) = parse_input("./input/day13/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&intersections, &carts);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {:?}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&intersections, &carts);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {:?}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}