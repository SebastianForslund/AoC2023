use core::panic;
use std::{
    collections::{btree_map::Keys, HashMap},
    f32::consts::E,
};

/* Part one */

fn p1_parse_string(input_str: &String) -> u32 {
    let mut res: String = String::new();

    let d1 = match input_str.chars().find(|c| c.is_digit(10)) {
        Some(res) => res,
        None => panic!("weird input"),
    };

    let d2 = match input_str.chars().rfind(|c| c.is_digit(10)) {
        Some(res) => res,
        None => panic!("weird input"),
    };

    res.push(d1);
    res.push(d2);

    println!("{}", res);

    return res.parse().unwrap();
}

pub fn d1p1(input: &Vec<String>) -> u32 {
    return input.iter().map(|str| p1_parse_string(str)).sum::<u32>();
}

/* Part two */

fn p2_parse_string(input_str: &String) -> u32 {
    let mut result: String = String::new();

    let first_primitive = match input_str.char_indices().find(|c| c.1.is_digit(10)) {
        Some(res) => res,
        None => panic!("weird input"),
    };
    let last_primitive = match input_str.char_indices().rfind(|c| c.1.is_digit(10)) {
        Some(res) => res,
        None => panic!("weird input"),
    };

    let spelled_digits: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("zero", '0'),
    ]);
    let mut found_spelled: Vec<(usize, char)> = Vec::new();

    for key in spelled_digits.keys() {
        let index = match input_str.find(key) {
            Some(index) => index,
            None => continue,
        };
        let index2 = match input_str.rfind(key) {
            Some(index2) => index2,
            None => continue,
        };

        found_spelled.push((index, spelled_digits[key]));
        found_spelled.push((index2, spelled_digits[key]));
    }

    let first_spelled = found_spelled.iter().min_by(|a, b| a.0.cmp(&b.0));
    match first_spelled {
        Some(first_spelled) => {
            if first_primitive.0 < first_spelled.0 {
                result.push(first_primitive.1)
            } else {
                result.push(first_spelled.1)
            }
        }
        None => result.push(first_primitive.1),
    };

    let last_spelled = found_spelled.iter().max_by(|a, b| a.0.cmp(&b.0));
    match last_spelled {
        Some(last_spelled) => {
            if last_primitive.0 > last_spelled.0 {
                result.push(last_primitive.1)
            } else {
                result.push(last_spelled.1)
            }
        }
        None => result.push(last_primitive.1),
    }

    println!("{}", result);

    return result.parse().unwrap();
}

pub fn d1p2(input: &Vec<String>) -> u32 {
    return input.iter().map(|str| p2_parse_string(str)).sum::<u32>();
}
