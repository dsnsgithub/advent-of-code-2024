use chrono::{Datelike, Utc};
use chrono_tz::America::New_York;

mod day1 { pub mod part1; pub mod part2; }
mod day2 { pub mod part1; pub mod part2; }
mod day3 { pub mod part1; pub mod part2; }
// mod day4 { pub mod part1; pub mod part2; }
// mod day5 { pub mod part1; pub mod part2; }
// mod day6 { pub mod part1; pub mod part2; }
// mod day7 { pub mod part1; pub mod part2; }
// mod day8 { pub mod part1; pub mod part2; }
// mod day9 { pub mod part1; pub mod part2; }
// mod day10 { pub mod part1; pub mod part2; }
// mod day11 { pub mod part1; pub mod part2; }
// mod day12 { pub mod part1; pub mod part2; }
// mod day13 { pub mod part1; pub mod part2; }
// mod day14 { pub mod part1; pub mod part2; }
// mod day15 { pub mod part1; pub mod part2; }
// mod day16 { pub mod part1; pub mod part2; }
// mod day17 { pub mod part1; pub mod part2; }
// mod day18 { pub mod part1; pub mod part2; }
// mod day19 { pub mod part1; pub mod part2; }
// mod day20 { pub mod part1; pub mod part2; }
// mod day21 { pub mod part1; pub mod part2; }
// mod day22 { pub mod part1; pub mod part2; }
// mod day23 { pub mod part1; pub mod part2; }
// mod day24 { pub mod part1; pub mod part2; }
// mod day25 { pub mod part1; pub mod part2; }
// mod day26 { pub mod part1; pub mod part2; }
// mod day27 { pub mod part1; pub mod part2; }
// mod day28 { pub mod part1; pub mod part2; }
// mod day29 { pub mod part1; pub mod part2; }
// mod day30 { pub mod part1; pub mod part2; }
// mod day31 { pub mod part1; pub mod part2; }

fn main() {
    let local_time = Utc::now().with_timezone(&New_York);
    let day = local_time.day() as usize;

	let day_modules: [[fn(); 2]; 3] = [
		[day1::part1::main, day1::part2::main],
		[day2::part1::main, day2::part2::main],
		[day3::part1::main, day3::part2::main],
		// [day4::part1::main, day4::part2::main],
		// [day5::part1::main, day5::part2::main],
		// [day6::part1::main, day6::part2::main],
		// [day7::part1::main, day7::part2::main],
		// [day8::part1::main, day8::part2::main],
		// [day9::part1::main, day9::part2::main],
		// [day10::part1::main, day10::part2::main],
		// [day11::part1::main, day11::part2::main],
		// [day12::part1::main, day12::part2::main],
		// [day13::part1::main, day13::part2::main],
		// [day14::part1::main, day14::part2::main],
		// [day15::part1::main, day15::part2::main],
		// [day16::part1::main, day16::part2::main],
		// [day17::part1::main, day17::part2::main],
		// [day18::part1::main, day18::part2::main],
		// [day19::part1::main, day19::part2::main],
		// [day20::part1::main, day20::part2::main],
		// [day21::part1::main, day21::part2::main],
		// [day22::part1::main, day22::part2::main],
		// [day23::part1::main, day23::part2::main],
		// [day24::part1::main, day24::part2::main],
		// [day25::part1::main, day25::part2::main],
		// [day26::part1::main, day26::part2::main],
		// [day27::part1::main, day27::part2::main],
		// [day28::part1::main, day28::part2::main],
		// [day29::part1::main, day29::part2::main],
		// [day30::part1::main, day30::part2::main],
		// [day31::part1::main, day31::part2::main],
	];

	println!("Day {}", day);

	println!("");

	println!("Part 1: ");
	day_modules[day - 1][0]();

	println!("");

	println!("Part 2: ");
	day_modules[day - 1][1]();
}
