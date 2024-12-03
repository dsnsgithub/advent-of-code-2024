use std::fs;

pub fn main() {
	let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let line_array = data.lines().collect::<Vec<_>>();

	let mut final_array = vec![];

	for line in line_array {
		let line_array = line.split(" ").collect::<Vec<_>>();

		final_array.push(line_array);
	}

	let mut sum = 0;

	for line in final_array {
		let mut workable = false;
		for i in 0..line.len() {
			let mut new_vec = line.to_vec();
			new_vec.remove(i);

			let lowest = new_vec[0].parse::<i32>().unwrap() < new_vec[1].parse::<i32>().unwrap();
			let mut broken = false;

			for i in 0..new_vec.len()-1 {
				let first = new_vec[i].parse::<i32>().unwrap();
				let second = new_vec[i+1].parse::<i32>().unwrap();
				if lowest && first > second {
					broken = true;
					break;
				}

				if !lowest && first < second {
					broken = true;
					break;
				}

				let distance=  (first - second).abs();
				if distance < 1 {
					broken = true;
					break;
				}

				if distance > 3 {
					broken = true;
					break;
				}
			}

			if !broken {
				workable = true;
				break;
			}
		}

		if workable {
			sum += 1;
		}
	}

	println!("{}", sum);
}