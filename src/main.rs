extern crate rand;

use std::str::FromStr;
use std::io;
use std::collections::HashMap;
use game::rps;
use game::weapon::{Weapon, get_rules};
use game::ai::CpuAi;

mod game;

fn main() {

    let mut ai = CpuAi::new();
    ai.add_weapon(Weapon::Rock);
    ai.add_weapon(Weapon::Paper);
    ai.add_weapon(Weapon::Scissors);

    println!("=============== Rock, Paper, Scissors ================");
    println!("Enter 'r' for Rock, 'p' for Paper, or 's' for Scissors");
    println!("------------------------------------------------------");
    loop {
        let mut input = String::new();
        let standard_rules = get_rules();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match Weapon::from_str(&input) {
            Ok(w)  => {
                ai.increment_weapon(&w);
                rps::play(&w, &ai, &standard_rules);
            },
            Err(_e) => {
                println!("Invalid input")
            },
        }
    }
}