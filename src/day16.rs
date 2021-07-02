use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Regs = Vec<usize>;
type Instruction = Vec<usize>;
type Observed = Vec<(Regs, Regs, Instruction)>;
type Program = Vec<Instruction>;

fn parse_input(path: &str) -> (Observed, Program) {
	let mut fstr = String::new();
	let mut ret_vec = Vec::new();
	let mut program = Vec::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	let first_spl = fstr.split("\n\n\n\n").collect::<Vec<&str>>();
	let second_spl = first_spl[0].split("\n\n").collect::<Vec<&str>>();
	for ins in second_spl {
		let spl = ins.split("\n").collect::<Vec<&str>>();
		let before = spl[0][9..19].split(", ").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
		let after = spl[2][9..19].split(", ").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
		let opcodes = spl[1].split(" ").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
		ret_vec.push((before, after, opcodes));
	}
	let prog_spl = first_spl[1].split("\n").collect::<Vec<&str>>();
	for line in prog_spl {
		let inst = line.split(" ").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
		program.push(inst);
	}
	(ret_vec, program)
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

fn part1(instructions: &Observed) -> usize {
	let mut num_passing = 0;
	
	for instr in instructions {
		let mut sum = 0;
		if addr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if addi(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if mulr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if muli(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if banr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if bani(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if borr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if bori(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if setr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if seti(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if gtir(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if gtri(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if gtrr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if eqir(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if eqri(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if eqrr(&instr.2, &instr.0) == instr.1 { sum += 1 };
		if sum >= 3 {
			num_passing += 1;
		}
	}
	num_passing
}

fn part2(instructions: &Observed, program: &Program) -> usize {
	let mut possibilities = Vec::new();
	let mut hs = HashSet::new();
	for i in 0..16 {
		hs.insert(i);
	}
	for i in 0..16 {
		possibilities.push(hs.clone());
	}
	for instr in instructions {
		if addr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&0); }
		if addi(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&1); }
		if mulr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&2); }
		if muli(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&3); }
		if banr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&4); }
		if bani(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&5); }
		if borr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&6); }
		if bori(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&7); }
		if setr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&8); }
		if seti(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&9); }
		if gtir(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&10); }
		if gtri(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&11); }
		if gtrr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&12); }
		if eqir(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&13); }
		if eqri(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&14); }
		if eqrr(&instr.2, &instr.0) != instr.1 { possibilities[instr.2[0]].remove(&15); }
	}

	let mut table = Vec::new();
	for _ in 0..16 {
		table.push(100);
	}

	while table.contains(&100) {
		for i in 0..possibilities.len() {
			if possibilities[i].len() == 1 && table[i] == 100 {
				let tmp = possibilities[i].iter().map(|v| *v).collect::<Vec<usize>>();
				table[i] = tmp[0];
				for j in 0..possibilities.len() {
					if possibilities[j].len() != 1 {
						possibilities[j].remove(&table[i]);
					}
				}
			}
		}
	}
	
	let mut regs = vec![0, 0, 0, 0];
	for inst in program {
		match table[inst[0]] {
			0 => regs = addr(&inst, &regs),
			1 => regs = addi(&inst, &regs),
			2 => regs = mulr(&inst, &regs),
			3 => regs = muli(&inst, &regs),
			4 => regs = banr(&inst, &regs),
			5 => regs = bani(&inst, &regs),
			6 => regs = borr(&inst, &regs),
			7 => regs = bori(&inst, &regs),
			8 => regs = setr(&inst, &regs),
			9 => regs = seti(&inst, &regs),
			10 => regs = gtir(&inst, &regs),
			11 => regs = gtri(&inst, &regs),
			12 => regs = gtrr(&inst, &regs),
			13 => regs = eqir(&inst, &regs),
			14 => regs = eqri(&inst, &regs),
			15 => regs = eqrr(&inst, &regs),
			_ => println!("INVALID OPCODE")
		}
	}
	regs[0]
}

pub fn main() {
	let (instructions, program) = parse_input("./input/day16/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&instructions);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&instructions, &program);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time); 
}