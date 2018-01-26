use rand;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Weapon {
    Rock,
    Paper,
    Scissors,
}

impl rand::Rand for Weapon {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => Weapon::Rock,
            1 => Weapon::Paper,
            2 => Weapon::Scissors,
            _ => panic!("whoops"),
        }
    }
}

impl ToString for Weapon {
    fn to_string(&self) -> String {
        match self {
            &Weapon::Rock     => "Rock".to_string(),
            &Weapon::Paper    => "Paper".to_string(),
            &Weapon::Scissors => "Scissors".to_string(),
        }
    }
}

impl FromStr for Weapon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'r' => Ok(Weapon::Rock),
            'p' => Ok(Weapon::Paper),
            's' => Ok(Weapon::Scissors),
            _   => Err(()),
        }
    }
}


pub fn get_rules() -> HashMap<Weapon, Vec<Weapon>> {
    let mut rules = HashMap::new();
    rules.insert(Weapon::Rock,     vec![Weapon::Scissors]);
    rules.insert(Weapon::Paper,    vec![Weapon::Rock]);
    rules.insert(Weapon::Scissors, vec![Weapon::Paper]);

    rules
}
