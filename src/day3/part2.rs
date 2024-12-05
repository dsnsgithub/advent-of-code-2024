use std::fs;

pub fn main() {
    let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let raw_line = data.lines().collect::<Vec<_>>()[0].to_string();
    let mut line = data.lines().collect::<Vec<_>>()[0].to_string();

    let do_indexes = raw_line.match_indices("do()").collect::<Vec<_>>();
    let do_not_indexes = raw_line.match_indices("don't()").collect::<Vec<_>>();

    // println!("{}, {}", do_indexes.len(), do_not_indexes.len());

    let mut sum = 0;
    let mut index_sum = 0;
    while line.find("mul(") != None && line.find(")") != None {
        let start_index: usize = line.find("mul(").unwrap();
        line.replace_range(0..start_index + 4, "");

        let mut closest_do_index: i32 = 0;
        let mut closest_do_not_index: i32 = 0;
        let true_index: usize = &start_index + index_sum;

        for (index, _) in &do_indexes {
            if index < &true_index {
                closest_do_index = *index as i32;
            }
        }

        for (index, _) in &do_not_indexes {
            if index < &true_index {
                closest_do_not_index = *index as i32;
            }
        }

        let end_index = line.find(")").unwrap();

        let new_line = line[0..end_index].to_string();

        if new_line.find(",") != None {
            let two_nums = new_line.split(",").collect::<Vec<_>>();

            if two_nums.len() == 2 {
                let first = two_nums[0].parse::<i32>();
                let second = two_nums[1].parse::<i32>();

                if Result::is_ok(&first) && Result::is_ok(&second) {
                    let first_num = first.unwrap();
                    let second_num = second.unwrap();

                    // println!("{}, {}, {}", closest_do_index, closest_do_not_index, true_index);
                    if closest_do_index >= closest_do_not_index {
                        // println!("DO: {}*{}", &first_num, &second_num);
                        sum += first_num * second_num;
                    } else {
                        // println!("DO NOT: {}*{}", &first_num, &second_num);
                    }
                }
            }
        }

        index_sum += start_index + 4;
    }

    println!("{}", sum);
}
