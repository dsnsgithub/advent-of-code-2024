use std::fs;

pub fn main() {
    let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let line_array = data.lines().collect::<Vec<_>>();

    let mut first_array = vec![];
    let mut second_array = vec![];

    let mut switch_order = false;
    for line in line_array {
        if line == "" {
            switch_order = true;
            continue;
        }

        if !switch_order {
            let line_array = line.split("|").collect::<Vec<_>>();
            first_array.push(line_array);
        } else {
            let line_array = line.split(",").collect::<Vec<_>>();
            second_array.push(line_array);
        }
    }

    let mut total = 0;
    for order in second_array {
        let mut valid_orders = 0;
        for (index, number) in order.iter().enumerate() {
            let mut sum = 0;

            for pair in &first_array {
                let first = pair[0].parse::<i32>().unwrap();

                if number.parse::<i32>().unwrap() == first {
                    if order.contains(&pair[1]) {
                        sum += 1
                    }
                }
            }

            if sum == order.len() - index - 1 {
                valid_orders += 1;
            }
        }

        if valid_orders == order.len() {
            total += order[order.len() / 2].parse::<i32>().unwrap();
        }
    }

    println!("{}", total);
}
