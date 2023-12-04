use std::collections::HashMap;

use crate::parsing::get_number_at;

fn fetch_owned_numbers(game: &String) -> Vec<u32> {
    let mut owned_numbers: Vec<u32> = Vec::new();

    for x in 0..10 {
        owned_numbers.push(get_number_at(game, x * 3 + 11).expect("owned number failed"));
    }

    return owned_numbers;
}

fn fetch_winning_numbers(game: &String) -> Vec<u32> {
    let mut winning_numbers: Vec<u32> = Vec::new();

    for x in 0..25 {
        winning_numbers.push(get_number_at(game, x * 3 + 43).expect("winning number failed"));
    }

    return winning_numbers;
}

fn p1_parse_game(game: &String) -> u32 {
    let card_nbr = get_number_at(game, 7).expect("card number failed");
    println!("Parsing game {}", game);
    let winning_numbers: Vec<u32> = fetch_winning_numbers(game);
    let owned_numbers: Vec<u32> = fetch_owned_numbers(game);

    println!("winning: ");
    winning_numbers.iter().for_each(|nbr| print!(" {}", nbr));

    println!();

    println!("owned: ");
    owned_numbers.iter().for_each(|nbr| print!(" {}", nbr));
    println!();

    let matched: Vec<&u32> = owned_numbers
        .iter()
        .filter(|owned| winning_numbers.contains(owned))
        .collect();

    let pow: i32 = matched.len() as i32 - 1;

    println!("  {} matches", pow + 1);

    if pow < 0 {
        return 0;
    }

    println!("  {} points", 2_u32.pow(pow as u32));

    return 2_u32.pow(pow as u32);
}

pub fn d4p1(input: Vec<String>) -> u32 {
    return input.iter().map(|str| p1_parse_game(str)).sum::<u32>();
}

struct Card {
    pub number: u32,
    pub matches: u32,
}

impl Card {
    fn new(line: &String) -> Card {
        let card_nbr = get_number_at(line, 7).expect("card number failed");
        //println!("Parsing game {}", line);
        let winning_numbers: Vec<u32> = fetch_winning_numbers(line);
        let owned_numbers: Vec<u32> = fetch_owned_numbers(line);
        let matched: Vec<&u32> = owned_numbers
            .iter()
            .filter(|owned| winning_numbers.contains(owned))
            .collect();

        return Card {
            number: card_nbr,
            matches: matched.len() as u32,
        };
    }
}

fn add_to_queue(game: &String, map: &mut HashMap<u32, u32>) {
    let origin: Card = Card::new(game);

    if origin.matches == 0 {
        return;
    }

    let power = *map.get(&origin.number).unwrap();
    println!(
        "'{}' Adding {}..{} with power {}",
        origin.number,
        origin.number + 1,
        origin.number + origin.matches,
        power
    );

    for m in (origin.number + 1)..=(origin.number + origin.matches) {
        map.insert(m, *map.get(&m).unwrap() + power);
    }
}

pub fn d4p2(input: &Vec<String>) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();

    for n in 1..=input.len() {
        map.insert(n as u32, 1);
    }

    for line in input {
        add_to_queue(&line, &mut map);
    }

    return map.into_iter().map(|f| f.1).sum();
}
