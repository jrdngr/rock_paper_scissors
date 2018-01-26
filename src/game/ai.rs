use std::collections::HashMap;
use rand;
use rand::Rng;
use super::weapon::Weapon;

pub struct CpuAi {
    player_choices: HashMap<Weapon, u64>,
}

impl CpuAi {
    pub fn new() -> CpuAi {
        CpuAi {
            player_choices: HashMap::new(),
        }
    }

    pub fn add_weapon(&mut self, weapon: Weapon) {
        self.player_choices.insert(weapon, 1);
    }

    pub fn increment_weapon(&mut self, weapon: &Weapon) -> u64 {
        let old_count = self.player_choices[&weapon];
        self.player_choices.insert(weapon.clone(), old_count + 1);
        self.player_choices[&weapon]
    }

    pub fn get_weapon_count(&self, weapon: &Weapon) -> u64 {
        self.player_choices[weapon]
    }

    pub fn get_weapon_weight(&self, weapon: &Weapon) -> f32 {
        let count = self.player_choices[&weapon];
        return count as f32 / self.get_count_total() as f32
    }

    pub fn get_count_total(&self) -> u64 {
        let mut total = 0;
        for (_, count) in &self.player_choices {
            total += count;
        }
        total
    }

    pub fn choose(&self) -> Weapon {
        rand::thread_rng().gen::<Weapon>()
    }
}