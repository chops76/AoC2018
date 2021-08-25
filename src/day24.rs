use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
struct Unit {
	id: usize,
	num_units: i32,
	hp: usize,
	damage: usize,
	damage_type: String,
	initiative: usize,
	weak: Vec<String>,
	immune: Vec<String>,
	infection: bool
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
		(self.num_units as usize * self.damage).cmp(&(other.num_units as usize * other.damage))
		    .then_with(|| self.initiative.cmp(&other.initiative))
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(path: &str, boost: usize) -> Vec<Unit> {
	let f = File::open(path).unwrap();
	let mut units = Vec::new();
	let mut is_infection = false;
	let mut count = 0;
	for l in BufReader::new(f).lines().flatten() {
		if l == "Immune System:" || l == "" {
			continue;
		} else if l == "Infection:" {
			is_infection = true;
			continue;
		}
		let spl = l.split(" with an ").collect::<Vec<&str>>();
		let left = spl[0].split(" ").collect::<Vec<&str>>();
		let right = spl[1].split(" ").collect::<Vec<&str>>();
		let mut damage = right[3].parse().unwrap();
		if !is_infection {
			damage += boost;
		}
		let mut new_unit = Unit { num_units: left[0].parse().unwrap(), hp: left[4].parse().unwrap(),
		                          damage: damage, damage_type: right[4].to_string(),
								  initiative: right[8].parse().unwrap(), infection: is_infection,
								  weak: Vec::new(), immune: Vec::new(), id: count };
		if left.len() != 7 {
			let spl2 = spl[0].split("(").collect::<Vec<&str>>();
			let attribs:&str = &spl2[1][..spl2[1].len()-1];
			for a in attribs.split("; ") {
				let spl = a.split(" to ").collect::<Vec<&str>>();
				for element in spl[1].split(", ") {
					if spl[0] == "weak" {
						new_unit.weak.push(element.to_string());
					} else {
						new_unit.immune.push(element.to_string());
					}
				}
			}
		}
		units.push(new_unit);
		count += 1;
	}

	units
}

fn part1(units: &Vec<Unit>) -> usize {
	let mut units = units.clone();
	let mut units_left = HashMap::new();
	for unit in &units {
		units_left.insert(unit.id, unit.num_units as i32);
	}

	loop {
		//println!("\n\n");
		for i in 0..units.len() {
			units.get_mut(i).unwrap().num_units = units_left[&units[i].id];
		}

		//println!("Immune System:");
		let mut on_inf = false;
		let mut inf_start = 0;
		for i in 0..units.len() {
			if units[i].num_units <= 0 {
				continue;
			}
			if units[i].infection && !on_inf {
				inf_start = i;
				on_inf = true;
				//println!("\nInfection:");
			}
			//println!("Group {} contains {} units.", i+1-inf_start, units[i].num_units);
		}
		//println!("");

		let mut order = BinaryHeap::new();
		let mut remaining = HashSet::new();
		let mut attacks = HashMap::new();
		for unit in &units {
			if units_left[&unit.id] <= 0 {
				continue;
			}
			order.push(unit);
			remaining.insert(unit);
		}

		let mut tmp = order.clone();
		//for unit in order.clone().into_sorted_vec() {
		let mut u = tmp.pop();
		while u != None {
			let unit = u.unwrap();
			//println!("{} {}", unit.num_units * unit.damage as i32, unit.initiative);
			let mut best: Option<&Unit> = None;
			let mut best_damage = 0;
			for enemy in &remaining {
				if unit.infection == !enemy.infection && !enemy.immune.contains(&unit.damage_type) {
					let mut damage = units_left[&unit.id] as usize * unit.damage;
					if enemy.weak.contains(&unit.damage_type) {
						damage *= 2;
					}
					if unit.infection {
						//println!("Infection group {} would deal defending group {} {} damage", unit.id-inf_start+1, enemy.id+1, damage);
					} else {
						//println!("Immunity group {} would attack defending group {} {} damage", unit.id+1, enemy.id-inf_start+1, damage);
					}
					if best == None || damage > best_damage || 
					(damage == best_damage && units_left[&enemy.id] as usize * enemy.damage > units_left[&best.unwrap().id] as usize * best.unwrap().damage) ||
					(damage == best_damage && units_left[&enemy.id] as usize * enemy.damage == units_left[&best.unwrap().id] as usize * best.unwrap().damage &&
						enemy.initiative > best.unwrap().initiative) {
							best_damage = damage;
							best = Some(enemy);
					}
				}
			}
			if best != None {
				remaining.remove(best.unwrap());
				attacks.insert(unit, best.unwrap());
			}
			u = tmp.pop();
		}

		//println!("");
		let mut something = order.iter().collect::<Vec<&&Unit>>();
		something.sort_by_key(|u| std::cmp::Reverse(u.initiative));
		for attacker in something {
			if attacks.contains_key(attacker) {
				let enemy = attacks[attacker];
				if units_left[&attacker.id] <= 0 || units_left[&enemy.id] <= 0 {
					continue;
				}

				let mut damage = units_left[&attacker.id] as usize * attacker.damage;
				if enemy.weak.contains(&attacker.damage_type) {
					damage *= 2;
				}
				if attacker.infection {
					//println!("Infection group {} attacks defending group {}, killing {} units", attacker.id-inf_start+1, enemy.id+1, damage/enemy.hp);
				} else {
					//println!("Immunity group {} attacks defending group {}, killing {} ints", attacker.id+1, enemy.id-inf_start+1, damage/enemy.hp);
				}
				*units_left.get_mut(&enemy.id).unwrap() -= (damage / enemy.hp) as i32;
			}
		}

		let mut inf_alive = 0;
		let mut imm_alive = 0;
		for u in &units {
			if u.infection && units_left[&u.id] > 0 {
				inf_alive += units_left[&u.id];
			}
			if !u.infection && units_left[&u.id] > 0 {
				imm_alive += units_left[&u.id];
			}
		}
		if inf_alive == 0 || imm_alive == 0 {
			if inf_alive != 0 {
				println!("Infection wins");
			} else {
				println!("Immunity wins");
			}
			return (inf_alive + imm_alive) as usize;
		}
		//println!("{:?}", units_left);
	}
}

pub fn main() {
	let units = parse_input("./input/day24/input.txt", 27);

	let p1_timer = Instant::now();
    let p1_result = part1(&units);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time); 
}