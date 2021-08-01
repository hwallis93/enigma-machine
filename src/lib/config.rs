use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RotorConfig {
    /// Which rotor, denoted by its number, from 1 - 8 inclusive
    pub number: usize,

    /// Ring setting, 0 - 25 inclusive
    pub setting: usize,

    /// Initial orientation of the rotor, denoted by the letter visible in the window
    pub window_letter: char,
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub left_rotor: RotorConfig,
    pub middle_rotor: RotorConfig,
    pub right_rotor: RotorConfig,
    pub plugboard: Vec<[char; 2]>,
}

#[derive(Clone, Copy, Debug)]
pub enum Slot {
    Left,
    Middle,
    Right,
}
type RotorSlotPair = (RotorConfig, Slot);

#[derive(Debug)]
pub enum ConfigError {
    RotorNumberOutsideRange(Slot, usize),
    TwoRotorsWithSameNumber(Slot, Slot, usize),
    RingSettingOutOfBounds(Slot, usize),
    InvalidWindowLetter(Slot, char),
    PlugboardDuplicateLetter(char),
    TooManyPlugs(usize),
}

impl Config {
    pub fn verify(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = vec![];

        errors.extend(self.verify_rotors());
        errors.extend(self.verify_plugboard());

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn verify_rotors(&self) -> Vec<ConfigError> {
        let mut errors = vec![];
        let left = (self.left_rotor, Slot::Left);
        let middle = (self.middle_rotor, Slot::Middle);
        let right = (self.right_rotor, Slot::Right);

        for rotor in [left, middle, right].iter() {
            errors.extend(self.verify_rotor_number_in_range(&rotor));
            errors.extend(self.verify_ring_settings(&rotor));
            errors.extend(self.verify_window_letters(&rotor));
        }
        errors.extend(self.verify_rotor_numbers_are_unique(&left, &middle));
        errors.extend(self.verify_rotor_numbers_are_unique(&left, &right));
        errors.extend(self.verify_rotor_numbers_are_unique(&middle, &right));

        errors
    }

    fn verify_rotor_numbers_are_unique(
        &self,
        (rotor1, slot1): &RotorSlotPair,
        (rotor2, slot2): &RotorSlotPair,
    ) -> Option<ConfigError> {
        if rotor1.number == rotor2.number {
            Some(ConfigError::TwoRotorsWithSameNumber(
                *slot1,
                *slot2,
                rotor1.number,
            ))
        } else {
            None
        }
    }

    fn verify_rotor_number_in_range(&self, (rotor, slot): &RotorSlotPair) -> Option<ConfigError> {
        if !(1..=8).contains(&rotor.number) {
            Some(ConfigError::RotorNumberOutsideRange(*slot, rotor.number))
        } else {
            None
        }
    }

    fn verify_ring_settings(&self, (rotor, slot): &RotorSlotPair) -> Option<ConfigError> {
        if !(0..=25).contains(&rotor.setting) {
            Some(ConfigError::RingSettingOutOfBounds(*slot, rotor.setting))
        } else {
            None
        }
    }

    fn verify_window_letters(&self, (rotor, slot): &RotorSlotPair) -> Option<ConfigError> {
        if !('A'..='Z').contains(&rotor.window_letter) {
            Some(ConfigError::InvalidWindowLetter(*slot, rotor.window_letter))
        } else {
            None
        }
    }

    fn verify_plugboard(&self) -> Vec<ConfigError> {
        let mut errors = vec![];
        let mut seen_letters: Vec<char> = vec![];
        let mut duplicated_letters: Vec<char> = vec![];

        for letter in self.plugboard.iter().flatten() {
            if seen_letters.contains(letter) {
                duplicated_letters.push(*letter)
            } else {
                seen_letters.push(*letter)
            }
        }
        duplicated_letters
            .iter()
            .unique()
            .for_each(|letter| errors.push(ConfigError::PlugboardDuplicateLetter(*letter)));

        if self.plugboard.len() > 10 {
            errors.push(ConfigError::TooManyPlugs(self.plugboard.len()))
        }

        errors
    }
}
