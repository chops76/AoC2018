use std::time::Instant;
use std::collections::HashMap;
use std::cmp;

fn part1() -> usize {
	let mut hm = HashMap::new();
	let mut scores = HashMap::new();
	let mut cur = 0;
	let mut player = 0;
	for i in 0..=71035 {
		hm.insert(i, (0,0));
	}
	for i in 0..479 {
		scores.insert(i, 0);
	}
	for i in 0..=71035 {
		if i == 0 || i % 23 != 0 {
			cur = hm[&cur].1;
			hm.insert(i, (cur, hm[&cur].1));
			hm.insert(cur, (hm[&cur].0, i));
			hm.insert(hm[&i].1, (i, hm[&hm[&i].1].1));
			cur = i;
		} else {
			let mut cur_score = scores[&player];
			cur_score += i;
			for _ in 0..7 {
				cur = hm[&cur].0;
			}
			cur_score += cur;
			scores.insert(player, cur_score);
			let prev = hm[&cur].0;
			let next = hm[&cur].1;

			hm.insert(prev, (hm[&prev].0, next));
			hm.insert(next, (prev, hm[&next].1));
			cur = next;
		}

		player += 1;
		player %= 479;
	}
	let mut max_score = 0;
	for (_, score) in scores {
		max_score = cmp::max(max_score, score);
	}
	max_score
}

fn part2() -> usize {
	let mut hm = HashMap::new();
	let mut scores = HashMap::new();
	let mut cur = 0;
	let mut player = 0;
	for i in 0..=7103500 {
		hm.insert(i, (0,0));
	}
	for i in 0..479 {
		scores.insert(i, 0);
	}
	for i in 0..=7103500 {
		if i == 0 || i % 23 != 0 {
			cur = hm[&cur].1;
			hm.insert(i, (cur, hm[&cur].1));
			hm.insert(cur, (hm[&cur].0, i));
			hm.insert(hm[&i].1, (i, hm[&hm[&i].1].1));
			cur = i;
		} else {
			let mut cur_score = scores[&player];
			cur_score += i;
			for _ in 0..7 {
				cur = hm[&cur].0;
			}
			cur_score += cur;
			scores.insert(player, cur_score);
			let prev = hm[&cur].0;
			let next = hm[&cur].1;

			hm.insert(prev, (hm[&prev].0, next));
			hm.insert(next, (prev, hm[&next].1));
			cur = next;
		}

		player += 1;
		player %= 479;
	}
	let mut max_score = 0;
	for (_, score) in scores {
		max_score = cmp::max(max_score, score);
	}
	max_score
}


pub fn main() {
	let p1_timer = Instant::now();
    let p1_result = part1();
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time); 
}