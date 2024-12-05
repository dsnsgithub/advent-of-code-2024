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
        let lowest = line[0].parse::<i32>().unwrap() < line[1].parse::<i32>().unwrap();
        let mut broken = false;

        for i in 0..line.len() - 1 {
            let first = line[i].parse::<i32>().unwrap();
            let second = line[i + 1].parse::<i32>().unwrap();
            if lowest && first > second {
                broken = true;
                break;
            }

            if !lowest && first < second {
                broken = true;
                break;
            }

            let distance = (first - second).abs();
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
            sum += 1;
        }
    }

    println!("{}", sum);
}
