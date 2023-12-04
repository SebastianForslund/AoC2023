use std::fs::*;
use std::io::{Error, Read};

pub fn get_input_as_vec(path_to_file: &str) -> Result<Vec<String>, Error> {
    let mut output: Vec<String> = Vec::new();
    let mut file: File = File::open(path_to_file)?;
    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);

    contents.lines().for_each(|ln| output.push(ln.into()));

    return Ok(output);
}

/* If index is on a number, parse the whole thing */
pub fn get_number_at(str: &String, index: usize) -> Option<u32> {
    let mut lower_bound = index;
    let mut upper_bound = index;

    if index >= str.len() {
        return None;
    }

    if !str.chars().nth(index)?.is_digit(10) {
        return None;
    }

    while lower_bound > 0 {
        match str.chars().nth(lower_bound - 1) {
            Some(x) => {
                if x.is_digit(10) {
                    lower_bound -= 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    while upper_bound < str.len() {
        match str.chars().nth(upper_bound + 1) {
            Some(x) => {
                if x.is_digit(10) {
                    upper_bound += 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    return Some(
        str[lower_bound..=upper_bound]
            .parse()
            .expect("parsing failed"),
    );
}
