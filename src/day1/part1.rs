use std::fs;

pub fn main() {
    let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let line_array = data.lines().collect::<Vec<_>>();

    let mut first_destinations: Vec<i64> = vec![];
    let mut second_destinations: Vec<i64> = vec![];

    for line in line_array {
        let line_array = line.split("   ").collect::<Vec<_>>();

        first_destinations.push(line_array[0].parse().unwrap());
        second_destinations.push(line_array[1].parse().unwrap());
    }

    first_destinations.sort();
    second_destinations.sort();
    
    let mut sum: i64 = 0;
    for (first, second) in first_destinations.iter().zip(second_destinations.iter()) {
        let distance = (first - second).abs();
        sum += distance;
        // println!("{}", distance);
    }

    println!("{}", sum);
}
