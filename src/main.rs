extern crate rand;
use rand::Rng;

use rand::seq::IteratorRandom;
use std::iter::FromIterator;

enum GroupType {
    CapitalLetters,
    LowerCase,
    Numbers,
    Symbols,
}

impl GroupType {
    fn init(self) -> &'static str {
        match self {
            GroupType::CapitalLetters => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            GroupType::LowerCase => "abcdefghijklmnopqrstuvwxyz",
            GroupType::Numbers => "0123456789",
            GroupType::Symbols => "!\"#$%&'()*+,-./:;<=>?@[]^_{|}~",
        }
    }
}

fn number2group(n: usize) -> GroupType {
    match n {
        0 => GroupType::CapitalLetters,
        1 => GroupType::LowerCase,
        2 => GroupType::Numbers,
        3 => GroupType::Symbols,
        _ => panic!("error"),
    }
}

fn take_n_chars(n: usize, group: GroupType) -> Vec<char> {
    let mut rng = rand::thread_rng();
    group.init().chars().choose_multiple(&mut rng, n)
}

fn main() {
    let pass_size = 20;
    let group_take_size = pass_size / 4;
    let mut v: Vec<Vec<char>> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..5 {
        let random_choice = rng.gen_range(0, 4);
        v.push(take_n_chars(group_take_size, number2group(random_choice)));
    }
    let v: Vec<char> = v.iter().flatten().cloned().collect();
    let password = String::from_iter(v);
    println!("password: {}", password);
}
