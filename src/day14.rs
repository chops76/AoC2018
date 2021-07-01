use std::time::Instant;

fn part1() -> Vec<usize> {
	let mut v = vec![3,7];
	let mut p1 = 0;
	let mut p2 = 1;
	let target_len = 939601 + 10;
	while v.len() < target_len {
		let sum = v[p1] + v[p2];
		if sum < 10 {
			v.push(sum);
		} else {
			v.push(1);
			v.push(sum % 10);
		}
		p1 = (p1 + 1 + v[p1]) % v.len();
		p2 = (p2 + 1 + v[p2]) % v.len();
	}
	if target_len < v.len() {
		return v[v.len()-11..v.len()-1].to_vec()
	}

	v[v.len()-10..v.len()].to_vec()
}

fn part2() -> usize {
	let mut v = vec![3,7];
	let mut p1 = 0;
	let mut p2 = 1;
	loop {
		let sum = v[p1] + v[p2];
		if sum < 10 {
			v.push(sum);
		} else {
			v.push(1);
			v.push(sum % 10);
		}
		p1 = (p1 + 1 + v[p1]) % v.len();
		p2 = (p2 + 1 + v[p2]) % v.len();

		if v.len() > 6 && v[v.len()-6..v.len()] == [9,3,9,6,0,1] {
			return v.len() - 6;
		}
		if v.len() > 7 && v[v.len()-7..v.len()-1] == [9,3,9,6,0,1] {
			return v.len() - 7;
		}
	}
}

pub fn main() {
	let p1_timer = Instant::now();
    let p1_result = part1();
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {:?}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {:?}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}