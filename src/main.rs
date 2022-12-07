use std::collections::{HashSet, HashMap, VecDeque};
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let file_name = "input/y2022q7.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let result = y2022q7(input);

    println!("{}", result);
}

// Given up
fn y2022q7(input: Vec<String>) -> i32 {
    let mut result = 0;

    let mut stack: VecDeque<String> = VecDeque::new();

    // let mut curr_dir: String = "root/".parse().unwrap();
    let mut hash_map: HashMap<String, i32> = HashMap::new();

    stack.push_front("/".parse().unwrap());

    for i in input {
        if i.starts_with("$ cd ") {
            let dir = i.split(" ").nth(2).unwrap().to_string();
            if dir == ".." {
                stack.pop_back();
                // let to_remove: String = curr_dir.chars().take(curr_dir.len() - 1).collect();
                //
                // let final_remv = to_remove.split("/").last().unwrap();
                // curr_dir = to_remove.strip_suffix(final_remv).unwrap().to_string();
            } else if dir != "/" {
                // curr_dir = curr_dir + dir + "/";
                stack.push_back(dir)
            }
        } else if i.starts_with("$ ls") {
            continue
        } else if i.starts_with("dir ") {
            continue
        } else if i.chars().nth(0).unwrap().is_numeric() {
            let value = i.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
            // println!("{}", value);
            for j in 0..=stack.len() {
                let mut final_path = "".to_string();
                for (idx, k) in stack.to_owned().into_iter().enumerate() {
                    let mut abd = "";
                    if idx == 0 {
                        abd = ""
                    } else {
                        abd = "/"
                    }
                    if idx <= j  {
                        final_path =  final_path.to_string() + k.as_str() + abd;
                    }
                }

                // println!("{}", final_path);


                // let mut new_stack = stack.clone().range(0..j);
                // let ab = new_stack.make_contiguous().join("/");
                //
                // println!("{} {}", ab, value);
                if !hash_map.contains_key(&*final_path)  {
                    hash_map.insert(final_path, value);
                    println!("{}", hash_map.get(&*"/".to_string()).unwrap());
                } else {
                    let mut abc  = final_path.clone().to_owned();
                    let mut val = hash_map.get(&*abc).unwrap();
                    hash_map.insert(abc,  val + value);
                    println!("{}", hash_map.get(&*"/".to_string()).unwrap());
                }
            }
        }
    }

    for &y in hash_map.values()  {
        println!("{}", y);
        if y < 100000 {
            result = result + y;
        }

    }

    result
}

fn y2022q5(input: Vec<String>) -> String {
    let mut result: String = format!("");

    let mut all_stacks: Vec<Vec<char>> = vec![vec!['D', 'L', 'V', 'T', 'M', 'H', 'F'],
        vec!['H', 'Q', 'G', 'J', 'C', 'T', 'N', 'P'],
        vec!['R', 'S', 'D', 'M', 'P', 'H'],
        vec!['L', 'B', 'V', 'F'],
        vec!['N', 'H', 'G', 'L', 'Q'],
        vec!['W', 'B', 'D', 'G', 'R', 'M', 'P'],
        vec!['G', 'M', 'N', 'R', 'C', 'H', 'L', 'Q'],
        vec!['C', 'L', 'W'],
        vec!['R', 'D', 'L', 'Q', 'J', 'Z', 'M', 'T'],
    ];

    for i in input {
        if i.contains("move") {
            let values: Vec<&str> = i.split(" ").collect();

            let src_stack = values[3].parse::<usize>().unwrap() - 1;
            let dest_stack = values[5].parse::<usize>().unwrap() - 1;
            let num = values[1].parse::<i32>().unwrap();

            let mut temp_stack: Vec<char> = Vec::new();
            for _ in 1..=num {
                // let option = all_stacks[src_stack].pop().unwrap();
                // all_stacks[dest_stack].push(option);

                let option = all_stacks[src_stack].pop().unwrap();
                temp_stack.push(option);
            }
            temp_stack.reverse();

            for k in temp_stack {
                all_stacks[dest_stack].push(k);
            }
        }
    }

    for mut x in all_stacks {
        result.push(x.pop().unwrap());
    }

    result
}

fn y2022q4a() ->  i32 {
    let mut result = 0;
    let mut result_b = 0;

    let file_name = "input/y2022q4.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for i in input {
        let (first, second) = i.split_once(",").unwrap();

        let (first_start, first_end) = first.split_once("-").unwrap();
        let (second_start, second_end) = second.split_once("-").unwrap();

        let first_start_num = first_start.parse::<i32>().unwrap();
        let first_end_num = first_end.parse::<i32>().unwrap();

        let second_start_num = second_start.parse::<i32>().unwrap();
        let second_end_num = second_end.parse::<i32>().unwrap();

        if first_start_num <= second_start_num && first_end_num >= second_end_num
            || second_start_num <= first_start_num && second_end_num >= first_end_num {
            result = result + 1
        }

        if first_start_num >= second_start_num && first_start_num <= second_end_num
            || second_start_num >= first_start_num && second_start_num <= first_end_num {
            result_b = result_b + 1
        }


    }

    result_b
}

fn y2022q3b() -> i32 {

    let mut result = 0;

    let file_name = "input/y2022q3.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut hash_map: HashMap<char, i32> = HashMap::new();

    for (idx, i) in input.iter().enumerate() {
        let set_f: HashSet<char> = i.chars().collect();
        for j in set_f {
            if hash_map.contains_key(&j) {
                hash_map.insert(j, hash_map.get(&j).unwrap() + 1);
            } else {
                hash_map.insert(j, 1);
            }
        }

        if idx % 3 == 2 {
            for (&x, &y) in hash_map.iter()  {
                println!("{} {}", x, y);
                if y == 3 {
                    if x.is_lowercase() {
                        result = result + (x as i32 - 96);
                    } else {
                        result = result + (x as i32 - 64) + 26;
                    }
                    break
                }
            }
            hash_map.clear();
        }
    }

    result
}

fn y2022q3a() -> i32 {

    let mut result = 0;

    let file_name = "input/y2022q2.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for i in input {
        let len = i.len() / 2;
        let first: String = i.chars().take(len).collect();
        let second: String = i.chars().skip(len).take(len).collect();

        let set_f: HashSet<char> = first.chars().collect();

        for j in second.chars() {
            if set_f.contains(&j) {
                if j.is_lowercase() {
                    result = result + (j as i32 - 96);
                } else {
                    result = result + (j as i32 - 64) + 26;
                }
                break
            }
        }
    }

    result
}

fn points_scored(opponent: &str, me: &str) -> i32 {
    let mut score = 0;

    if me == "X" {
        score = score + 1;
        if opponent == "A" {
            score = score + 3;
        } else if opponent == "C" {
            score = score + 6
        }
    } else if  me == "Y" {
        score = score + 2;
        if opponent == "B" {
            score = score + 3;
        } else if opponent == "A" {
            score = score + 6
        }
    } else {
        score = score + 3;
        if opponent == "C" {
            score = score + 3;
        } else if opponent == "B" {
            score = score + 6
        }
    }

    score

}

fn points_scoreds2(opponent: &str, me: &str) -> i32 {
    let mut score = 0;

    if opponent == "A" {
        if me == "X" {
            score = score + 3;
            score = score + 0;
        } else if me == "Y" {
            score = score + 1;
            score = score + 3;
        } else {
            score = score + 2;
            score = score + 6;
        }
    } else if  opponent == "B" {
        if me == "X" {
            score = score + 1;
            score = score + 0;
        } else if me == "Y" {
            score = score + 2;
            score = score + 3;
        } else {
            score = score + 3;
            score = score + 6;
        }
    } else {
        if me == "X" {
            score = score + 2;
            score = score + 0;
        } else if me == "Y" {
            score = score + 3;
            score = score + 3;
        } else {
            score = score + 1;
            score = score + 6;
        }
    }

    score

}


fn y2022q2a() -> i32 {
    let mut score = 0;

    let file_name = "input/y2022q2.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for i in input {
        let (opponent, me) = i.split_once(" ").unwrap();
        score = score + points_scoreds2(opponent, me);
    }

    score
}

fn y2022q1a() -> i32 {
    let file_name = "input/y2022q1a.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    let mut elf_num = 1;
    for i in input {
        if i.is_empty() {
            elf_num = elf_num + 1;
        } else {
            if hash_map.contains_key(&elf_num) {
                hash_map.insert(elf_num, hash_map.get(&elf_num).unwrap() + i.parse::<i32>().unwrap());
            } else {
                hash_map.insert(elf_num, i.parse::<i32>().unwrap());
            }
        }
    }

    let mut values: Vec<i32> = hash_map.into_values().collect();
    values.sort_by(|a, b| b.cmp(a));


    // my dumb ass thinking i have to find the elf number
    // let mut result = 1;
    // let mut result_cal = 0;

    // for (&elf, &calories) in hash_map.iter() {
    //     println!("fdd {} f  {}", calories, elf);
    //
    //     if calories >= result_cal {
    //         result_cal = calories;
    //         result = elf;
    //     }
    // }

    values[0] + values[1] + values[2]
}

fn y2020() -> i32 {
    let file_name = "input/y2020q1a.txt";
    println!("Reading file {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let input: Vec<i32> = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    let mut hash_set: HashSet<i32> = HashSet::new();

    let target = 2020;

    for i in input {
        let remaining_sum = target - i;
        if hash_set.contains(&remaining_sum) {
            return i * remaining_sum;
        }
        hash_set.insert(i);
    }
    0
}
