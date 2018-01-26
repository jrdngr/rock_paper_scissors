use std::collections::HashMap;
use super::weapon::{Weapon};
use super::ai::CpuAi;

pub enum Winner {
    Player,
    Cpu,
    Draw,
}

pub fn play(player_choice: &Weapon, ai: &CpuAi, rules: &HashMap<Weapon, Vec<Weapon>>) {
    let cpu_choice = ai.choose();
    println!("Player: {:?}  Cpu: {:?}", &player_choice, &cpu_choice);
    match get_winner(&player_choice, &cpu_choice, &rules) {
        Winner::Player => println!("You win!\n"),
        Winner::Cpu    => println!("You lose!\n"),
        Winner::Draw   => println!("It's a draw\n"),
    }
}

fn get_winner(player_choice: &Weapon, cpu_choice: &Weapon, rules: &HashMap<Weapon, Vec<Weapon>>) -> Winner {
    if &player_choice == &cpu_choice {
        return Winner::Draw;
    } else {
        let player_choice_rule = &rules[&player_choice];
        if player_choice_rule.contains(&cpu_choice) {
            return Winner::Player;
        } else {
            return Winner::Cpu;
        }
    }
}