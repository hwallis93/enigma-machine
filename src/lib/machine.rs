use crate::{
    config::{Config, ConfigError},
    constants::REFLECTOR_B,
    rotor::{Rotor, RotorPosition},
    utils::{alphabetical_index, alphabetical_letter},
};

/// Emulates an enigma machine
///
/// Pass a `Config` struct to `new()` to build the machine. Then encode letters with `encode()`
pub struct EnigmaMachine {
    right_rotor: Rotor,
    middle_rotor: Rotor,
    left_rotor: Rotor,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    pub fn new(config: Config) -> Result<Self, Vec<ConfigError>> {
        config.verify()?;

        Ok(EnigmaMachine {
            right_rotor: Rotor::new(config.right_rotor),
            middle_rotor: Rotor::new(config.middle_rotor),
            left_rotor: Rotor::new(config.left_rotor),
            plugboard: Plugboard {
                wire_pairs: config.plugboard,
            },
        })
    }

    /// Encodes a single letter in the range 'A'..'Z'
    pub fn encode(&mut self, input: char) -> char {
        self.advance_rotors();

        let plugboard_output = alphabetical_index(&self.plugboard.encode(input));
        let rotor1_output = self.right_rotor.encode(plugboard_output);
        let rotor2_output = self.middle_rotor.encode(rotor1_output);
        let rotor3_output = self.left_rotor.encode(rotor2_output);
        let reflector_output = self.reflect(rotor3_output);
        let rotor3_backwards_output = self.left_rotor.encode_backwards(reflector_output);
        let rotor2_backwards_output = self.middle_rotor.encode_backwards(rotor3_backwards_output);
        let rotor1_backwards_output = self.right_rotor.encode_backwards(rotor2_backwards_output);
        self.plugboard
            .encode(alphabetical_letter(rotor1_backwards_output))
    }

    /// Rotates all necessary rotors
    fn advance_rotors(&mut self) {
        let right_rotor_is_on_notch = self.right_rotor.is_on_notch();
        let middle_rotor_is_on_notch = self.middle_rotor.is_on_notch();

        self.right_rotor.rotate();
        if middle_rotor_is_on_notch || right_rotor_is_on_notch {
            self.middle_rotor.rotate();
        }
        if middle_rotor_is_on_notch {
            self.left_rotor.rotate();
        }
    }

    /// Passes a letter through reflector B
    fn reflect(&self, input: RotorPosition) -> RotorPosition {
        alphabetical_index(&REFLECTOR_B[input])
    }
}

struct Plugboard {
    pub wire_pairs: Vec<[char; 2]>,
}

impl Plugboard {
    /// Pass a letter through the plugboard
    fn encode(&self, input: char) -> char {
        for [letter1, letter2] in self.wire_pairs.iter() {
            if letter1 == &input {
                return *letter2;
            }
            if letter2 == &input {
                return *letter1;
            }
        }
        input
    }
}
