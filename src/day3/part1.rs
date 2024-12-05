use std::fs;
// use regex::Regex;

pub fn main() {
    let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let mut line = data.lines().collect::<Vec<_>>()[0].to_string();

    // let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    // let matches = re.find_iter(&line).collect::<Vec<_>>();

    // let mut sum = 0;
    // for test in matches {
    //     let test_str = test.as_str();

    //     let two_nums = test_str.replace("mul(", "").replace(")", "");
    //     let two_nums_array = two_nums.split(",").collect::<Vec<_>>();

    //     let first_num: u32 = two_nums_array[0].parse().unwrap();
    //     let second_num: u32 = two_nums_array[1].parse().unwrap();

    //     sum += first_num * second_num
    // }

    // println!("{}", sum)

    let mut sum = 0;
    while line.find("mul(") != None && line.find(")") != None {
        let start_index = line.find("mul(").unwrap();
        line.replace_range(0..start_index + 4, "");

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

                    // println!("{}*{}", &first_num, &second_num);
                    sum += first_num * second_num;
                }
            }
        }
    }

    println!("{}", sum);
}
