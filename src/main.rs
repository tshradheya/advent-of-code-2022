use std::collections::{HashSet, HashMap};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ptr::hash;
use std::thread::scope;


fn main() {
    let result = y2022q3b();

    println!("{}", result);
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
        let setF: HashSet<char> = i.chars().collect();
        for j in setF {
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

        let setF: HashSet<char> = first.chars().collect();

        for j in second.chars() {
            if setF.contains(&j) {
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

fn pointsScored(opponent: &str, me: &str) -> i32 {
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

fn pointsScoreds2(opponent: &str, me: &str) -> i32 {
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
        score = score + pointsScoreds2(opponent, me);
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

    let mut elfNum = 1;
    for i in input {
        if i.is_empty() {
            elfNum = elfNum + 1;
        } else {
            if hash_map.contains_key(&elfNum) {
                hash_map.insert(elfNum, hash_map.get(&elfNum).unwrap() + i.parse::<i32>().unwrap());
            } else {
                hash_map.insert(elfNum, i.parse::<i32>().unwrap());
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
