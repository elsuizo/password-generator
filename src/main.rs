extern crate rand;
use rand::Rng;

use std::env;

use rand::seq::IteratorRandom;
use std::iter::FromIterator;

/// The diferents source of letters
enum GroupType {
    CapitalLetters,
    LowerCase,
    Numbers,
    Symbols,
}

#[derive(Debug)]
struct UserInput {
    password_lenght: usize,
}

impl UserInput {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("the number of parameters are two")
        }
        if let Ok(password_lenght) = args[1].parse() {
            Ok(UserInput{password_lenght})
        } else {
            Err("wrong number of parameters")
        }
    }
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

fn parse_user_input(args: &[String]) -> UserInput {
    UserInput::new(args).expect("no se pudo parsear los parametros")
}

// NOTE(elsuizo:2020-05-11): o sea que si le pasamos dos args en realidad son tres
fn main() {

    let args: Vec<String> = env::args().collect();
    let params = parse_user_input(&args);
    if params.password_lenght < 4 {
        panic!("error the password len must be greater than 4");
    }
    println!("params: {}", params.password_lenght);
    let group_take_size = (params.password_lenght / 4) + params.password_lenght % 4;
    println!("group_take_size: {:}", group_take_size);
    let mut v: Vec<Vec<char>> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..group_take_size {
        let random_choice = rng.gen_range(0, 4);
        v.push(take_n_chars(group_take_size, number2group(random_choice)));
    }

    let v: Vec<char> = v.iter().flatten().cloned().collect();
    let password = String::from_iter(v);
    println!("password: {}", password);
}
