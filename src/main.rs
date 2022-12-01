use std::collections::{HashSet, HashMap};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let result = y2022q1a();

    println!("{}", result);
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
