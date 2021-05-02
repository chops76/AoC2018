use std::time::Instant;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
	let days = [
		day1::main,
		day2::main,
		day3::main,
		day4::main,
		day5::main,
		day6::main
	];

	let args: Vec<String> = env::args().collect();
	
	if args.len() > 1 {
		days[args[1].parse::<usize>().unwrap()-1]();
	} else {
		let total_timer = Instant::now();

		for (day, main) in days.iter().enumerate() {
			println!( "--------------------------" );
			println!( "Day {}",	day + 1	);
			println!( "--------------------------" );
			main();
			println!();
		}

		let total_time = total_timer.elapsed();
		println!("Total Time: {:?}", total_time);
	}
}
