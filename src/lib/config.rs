use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RotorConfig {
    pub number: usize,
    pub setting: usize,
    pub window_letter: char,
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub left_rotor: RotorConfig,
    pub middle_rotor: RotorConfig,
    pub right_rotor: RotorConfig,
    pub plugboard: Vec<[char; 2]>,
}

impl Config {
    pub fn verify(&self) -> Result<(), String> {
        let valid_rotor_range = 1..8;

        // Valid rotor numbers
        if !valid_rotor_range.contains(&self.left_rotor.number) {
            return Err(format!(
                "Rotors may only have values 1-7. Left rotor had value {}",
                self.left_rotor.number
            ));
        }
        if !valid_rotor_range.contains(&self.middle_rotor.number) {
            return Err(format!(
                "Rotors may only have values 1-7. Middle rotor had value {}",
                self.middle_rotor.number
            ));
        }
        if !valid_rotor_range.contains(&self.right_rotor.number) {
            return Err(format!(
                "Rotors may only have values 1-7. Right rotor had value {}",
                self.right_rotor.number
            ));
        }

        // No repeat rotors
        if self.left_rotor.number == self.middle_rotor.number {
            return Err(format!(
                "Left and Middle rotors both had value {}. Each rotor can only be used once.",
                self.left_rotor.number
            ));
        }
        if self.left_rotor.number == self.right_rotor.number {
            return Err(format!(
                "Left and Right rotors both had value {}. Each rotor can only be used once.",
                self.left_rotor.number
            ));
        }
        if self.middle_rotor.number == self.right_rotor.number {
            return Err(format!(
                "Middle and Right rotors both had value {}. Each rotor can only be used once.",
                self.middle_rotor.number
            ));
        }

        // No repeats in plugboard
        let mut unique_letters: Vec<char> = vec![];
        for letter in self.plugboard.iter().flatten() {
            if unique_letters.contains(letter) {
                return Err(format!(
                    "Cannot have letter in plugboard pairs more than once. Found '{}' more than once",
                    letter
                ));
            } else {
                unique_letters.push(*letter)
            }
        }
        // 0 - 10 plugs in plugboard
        if self.plugboard.len() > 10 {
            return Err(format!(
                "Cannot have more than 10 pairs in plugboard. Found {} pairs",
                self.plugboard.len()
            ));
        }
        Ok(())
    }
}
