use std::thread::current;

use crate::parsing::get_number_at;

fn is_part_symbol(symbol: &char) -> bool {
    return !symbol.is_digit(10) && *symbol != '.';
}

fn access(mtx: &Vec<String>, x: i32, y: i32) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }

    let x_sz: usize = x as usize;
    let y_sz: usize = y as usize;

    /* Perform the bounds checking here so we can just unwrap later */
    if y_sz >= mtx.len() {
        return None;
    }

    if x_sz >= mtx[y_sz].chars().count() {
        return None;
    }

    /* all is good in the hood */
    return Some(
        mtx[y_sz]
            .chars()
            .nth(x_sz)
            .expect("my bounds checking sucks"),
    );
}

fn access_nbr(mtx: &Vec<String>, x: i32, y: i32) -> Option<char> {
    return match access(mtx, x, y) {
        Some(c) => {
            if c.is_digit(10) {
                Some(c)
            } else {
                None
            }
        }
        None => None,
    };
}

/* Check if a character has a part associated with it (do this for every digit) */
fn has_part(mtx: &Vec<String>, x: i32, y: i32) -> bool {
    for x_iter in x - 1..=x + 1 {
        for y_iter in y - 1..=y + 1 {
            let c = match access(mtx, x_iter, y_iter) {
                Some(c) => c,
                None => continue,
            };

            if is_part_symbol(&c) {
                return true;
            }
        }
    }

    return false;
}

fn p1_line_func(mtx: &Vec<String>, line: &String, y: usize) -> Vec<u32> {
    let mut numbers: Vec<String> = Vec::new();
    let mut current_number: String = String::new();
    let mut found_part: bool = false;

    for e in line.char_indices() {
        let c = e.1;
        let x = e.0;

        /* This doesnt work if the number is the last on the line...
         * ..... whatever just add them to the input text arrest me
         * */
        if !c.is_digit(10) {
            if !current_number.is_empty() {
                if found_part {
                    numbers.push(current_number);
                    found_part = false;
                }
                current_number = String::new();
            }
            continue;
        } else {
            if has_part(mtx, x as i32, y as i32) {
                found_part = true;
            }
            current_number.push(c);
        }
    }

    println!("Line {}:", y + 1);
    for num in &numbers {
        println!("{}", num);
    }

    /* i still have to convert them to numbers... wow that was easy i love functional style programming */
    return numbers.iter().map(|str| str.parse().unwrap()).collect();
}

fn check_row(mtx: &Vec<String>, x: usize, y: usize, numbers: &mut Vec<u32>) {
    /* theres a nicer way to do this... i dont care */
    let left = access_nbr(mtx, x as i32 - 1, y as i32);
    let mid = access_nbr(mtx, x as i32, y as i32);
    let right = access_nbr(mtx, x as i32 + 1, y as i32);

    if mid.is_some() {
        numbers.push(get_number_at(&mtx[y], x).unwrap());
    } else {
        if left.is_some() {
            numbers.push(get_number_at(&mtx[y], x - 1).unwrap());
        }

        if right.is_some() {
            numbers.push(get_number_at(&mtx[y], x + 1).unwrap());
        }
    }
}

/* Return all adjacent numbers */
fn get_adjacent_numbers(mtx: &Vec<String>, x: usize, y: usize) -> Vec<u32> {
    let mut adjacent_numbers: Vec<u32> = Vec::new();

    /* Bottom and top row */
    check_row(mtx, x, y + 1, &mut adjacent_numbers);
    check_row(mtx, x, y - 1, &mut adjacent_numbers);

    /* Check the right */
    let right_nbr = get_number_at(&mtx[y], x + 1);
    if right_nbr.is_some() {
        adjacent_numbers.push(right_nbr.expect("checked"));
    }

    /* Check the left */
    let left_nbr = get_number_at(&mtx[y], x - 1);
    if left_nbr.is_some() {
        adjacent_numbers.push(left_nbr.expect("checked"));
    }

    return adjacent_numbers;
}

fn p2_line_func(mtx: &Vec<String>, line: &String, y: usize) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    for e in line.char_indices() {
        let c = e.1;
        let x = e.0;

        /* Look around each gear */
        if c == '*' {
            println!("getting adjacent");
            let adjacent = get_adjacent_numbers(mtx, x, y);
            if adjacent.len() == 2 {
                numbers.push(adjacent[0] * adjacent[1]);
            }
        }
    }

    println!("Line {}:", y + 1);

    return numbers;
}

pub fn d3p1(input: Vec<String>) -> u32 {
    /* LIDL unit tests B) */
    assert!(access(&input, 0, 0).is_some());
    assert!(access(&input, 9999, 0).is_none());
    assert!(access(&input, -1, 0).is_none());

    assert!(!is_part_symbol(&'.'));
    assert!(!is_part_symbol(&'5'));
    assert!(is_part_symbol(&'&'));
    assert!(is_part_symbol(&'*'));

    let mut total: u32 = 0;

    for line in input.iter().enumerate() {
        total += p1_line_func(&input, line.1, line.0).iter().sum::<u32>();
    }

    return total;
}

pub fn d3p2(input: Vec<String>) -> u32 {
    /* Wow.. parsing by digit was hard in the last one, this time its gonna be impossble */
    /* Iterate over all gears instead... duh */

    let mut total: u32 = 0;

    for line in input.iter().enumerate() {
        total += p2_line_func(&input, line.1, line.0).iter().sum::<u32>();
    }

    return total;
}
