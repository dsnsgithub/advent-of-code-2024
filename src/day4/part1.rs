use std::fs;

fn check_line(
    array: &Vec<Vec<&str>>,
    x: i32,
    y: i32,
    x_offset: i32,
    y_offset: i32,
    string: &str,
) -> bool {
    if string == "" {
        return true;
    }

    if (y >= 0 && y < array.len() as i32) && (x >= 0 && x < array[y as usize].len() as i32) {
        if string.starts_with(array[y as usize][x as usize]) {
            return check_line(
                array,
                x + x_offset,
                y + y_offset,
                x_offset,
                y_offset,
                string.split_at(1).1,
            );
        }
    }

    return false;
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
            if final_array[i][j] == "X" {
                for x_offset in -1..=1 {
                    for y_offset in -1..=1 {
                        if check_line(&final_array, j as i32, i as i32, x_offset, y_offset, "XMAS")
                        {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", sum);
}
