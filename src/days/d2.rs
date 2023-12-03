use std::{collections::HashMap, fmt::format};

/* Return number found in string (make sure theres just 1 number in the string) */
fn parse_number(str: &String) -> u32 {
    let nbr_str: String = str.chars().filter(|c| c.is_digit(10)).collect();
    return nbr_str.parse().expect("didnt find a number i guess");
}

fn get_game(str: &mut String) -> u32 {
    let index = str.find(":").expect("no colon in input wtf");
    let game_str = &str[0..index].to_string();
    let result = parse_number(game_str);

    /* Get rid of this part of this string since we dont need it */
    str.replace_range(0..index + 1, "");

    return result;
}

fn get_color_pairs(str: &mut String) -> Vec<&str> {
    let split: Vec<&str> = str.split(",").collect();
    let split: Vec<&str> = split.iter().map(|s| s.trim()).collect();
    return split;
}

fn populate_map_with_pair(map: &mut HashMap<&str, u32>, pair: &str) {
    let pair_str = pair.to_string();
    let pair_val = parse_number(&pair_str);

    let mut current_val: u32;

    for key in map.keys() {
        if pair_str.contains(key) {
            current_val = map.get(key).unwrap().to_owned();
            if pair_val > current_val {
                map.insert(key, pair_val);
            }
            break;
        }
    }
}

fn p2(str: &String) -> u32 {
    /* Assume each color is in every game */
    let mut color_vals: HashMap<&str, u32> = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);

    let mut our_str = str.to_owned();
    let game_nbr = get_game(&mut our_str);
    println!("Game: {}", game_nbr);

    let mut our_str = our_str.replace(';', ",");

    let pairs = get_color_pairs(&mut our_str);
    pairs
        .iter()
        .for_each(|pair| populate_map_with_pair(&mut color_vals, pair));
    let mut result = 1;
    color_vals.iter().for_each(|p| println!("{}: {}", p.0, p.1));
    color_vals.values().for_each(|v| result *= v);
    println!("result: {}", result);
    return result;
}

fn p1(str: &String) -> u32 {
    let mut our_str = str.to_owned();
    let game_nbr = get_game(&mut our_str);
    println!("Game: {}", game_nbr);

    let mut our_str = our_str.replace(';', ",");

    let pairs = get_color_pairs(&mut our_str);
    for pair in pairs {
        let pair_val = parse_number(&pair.to_string());

        if pair.contains("red") {
            if pair_val > 12 {
                return 0;
            }
        }

        if pair.contains("blue") {
            if pair_val > 14 {
                return 0;
            }
        }

        if pair.contains("green") {
            if pair_val > 13 {
                return 0;
            }
        }
    }

    return game_nbr;
}

pub fn d2p1(input: &mut Vec<String>) -> u32 {
    return input.iter().map(|str| p2(str)).sum::<u32>();
}
