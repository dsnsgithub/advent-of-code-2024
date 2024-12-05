use std::fs;

fn safe(array: &Vec<Vec<&str>>, x: i32, y: i32, string: &str) -> bool {
    return (y >= 0 && y < array.len() as i32)
        && (x >= 0 && x < array[y as usize].len() as i32)
        && array[y as usize][x as usize] == string;
}

pub fn main() {
    let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let line_array = data.lines().collect::<Vec<_>>();

    let mut final_array: Vec<Vec<&str>> = vec![];

    for line in line_array {
        let mut line_array = line.trim().split("").collect::<Vec<_>>();

        line_array.remove(0);
        line_array.remove(line_array.len() - 1);

        final_array.push(line_array);
    }

    let mut sum = 0;
    for i in 0..final_array.len() {
        for j in 0..final_array[i].len() {
            if final_array[i][j] == "A" {
                let mut inner_sum = 0;
                if safe(&final_array, j as i32 - 1, i as i32 - 1, "M") {
                    if safe(&final_array, j as i32 + 1, i as i32 + 1, "S") {
                        inner_sum += 1;
                    }
                }

                if safe(&final_array, j as i32 - 1, i as i32 + 1, "M") {
                    if safe(&final_array, j as i32 + 1, i as i32 - 1, "S") {
                        inner_sum += 1;
                    }
                }

                if safe(&final_array, j as i32 + 1, i as i32 - 1, "M") {
                    if safe(&final_array, j as i32 - 1, i as i32 + 1, "S") {
                        inner_sum += 1;
                    }
                }

                if safe(&final_array, j as i32 + 1, i as i32 + 1, "M") {
                    if safe(&final_array, j as i32 - 1, i as i32 - 1, "S") {
                        inner_sum += 1;
                    }
                }

                sum += inner_sum / 2;
            }
        }
    }

    println!("{}", sum);
}
