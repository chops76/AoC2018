use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;

type Regs = Vec<usize>;
type Instruction = Vec<usize>;
type Program = Vec<(String, Instruction)>;

fn parse_input(path: &str) -> Program {
	let mut program = Vec::new();

	let f = File::open(path).unwrap();
 	for line in BufReader::new(f).lines().flatten() {
		let spl = line.split(" ").collect::<Vec<&str>>();
		program.push((spl[0].to_string(), vec![0, spl[1].parse().unwrap(), spl[2].parse().unwrap(), spl[3].parse().unwrap()]));
	}
	program
}

fn addr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] + regs[instr[2]];

	after
}

fn addi(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] + instr[2];

	after
}

fn mulr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] * regs[instr[2]];

	after
}

fn muli(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] * instr[2];

	after
}

fn banr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] & regs[instr[2]];

	after
}

fn bani(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] & instr[2];

	after
}

fn borr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] | regs[instr[2]];

	after
}

fn bori(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]] | instr[2];

	after
}

fn setr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = regs[instr[1]];

	after
}

fn seti(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = instr[1];

	after
}

fn gtir(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = if instr[1] > regs[instr[2]] { 1 } else { 0 };

	after
}

fn gtri(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = if regs[instr[1]] > instr[2] { 1 } else { 0 };

	after
}

fn gtrr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = if regs[instr[1]] > regs[instr[2]] { 1 } else { 0 };

	after
}

fn eqir(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = if instr[1] == regs[instr[2]] { 1 } else { 0 };

	after
}

fn eqri(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = if regs[instr[1]] == instr[2] { 1 } else { 0 };

	after
}

fn eqrr(instr: &Instruction, regs: &Regs) -> Regs {
	let mut after = regs.clone();
	after[instr[3]] = if regs[instr[1]] == regs[instr[2]] { 1 } else { 0 };

	after
}

fn part1(program: &Program) -> usize {
	let mut regs = vec![0, 0, 0, 0, 0, 0];
	
	while regs[5] < program.len()  {
		match program[regs[5]].0.as_str() {
			"setr" => regs = setr(&program[regs[5]].1, &regs),
			"eqrr" => regs = eqrr(&program[regs[5]].1, &regs),
			"gtri" => regs = gtri(&program[regs[5]].1, &regs),
			"muli" => regs = muli(&program[regs[5]].1, &regs),
			"eqir" => regs = eqir(&program[regs[5]].1, &regs),
			"borr" => regs = borr(&program[regs[5]].1, &regs),
			"bori" => regs = bori(&program[regs[5]].1, &regs),
			"mulr" => regs = mulr(&program[regs[5]].1, &regs),
			"gtrr" => regs = gtrr(&program[regs[5]].1, &regs),
			"seti" => regs = seti(&program[regs[5]].1, &regs),
			"banr" => regs = banr(&program[regs[5]].1, &regs),
			"eqri" => regs = eqri(&program[regs[5]].1, &regs),
			"addr" => regs = addr(&program[regs[5]].1, &regs),
			"gtir" => regs = gtir(&program[regs[5]].1, &regs),
			"addi" => regs = addi(&program[regs[5]].1, &regs),
			"bani" => regs = bani(&program[regs[5]].1, &regs),
			_ => println!("INVALID OPCODE")
		}
		regs[5] += 1;
	}
	regs[0]
}

fn part2() -> usize {
	let mut total = 0;
	for i in 1..=10551373 {
		if 10551373 % i == 0 {
			total += i;
		}
	}
	total
}

pub fn main() {
	let instructions = parse_input("./input/day19/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&instructions);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time); 

	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}