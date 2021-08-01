use crate::{
    config::Config,
    constants::REFLECTOR_B,
    rotor::{Rotor, RotorPosition},
    utils::{alphabetical_index, alphabetical_letter},
};

pub struct EnigmaMachine {
    rotors: [Rotor; 3],
    plugboard: Plugboard,
}

impl EnigmaMachine {
    pub fn new(config: Config) -> Self {
        EnigmaMachine {
            rotors: [
                Rotor::new(config.right_rotor),
                Rotor::new(config.middle_rotor),
                Rotor::new(config.left_rotor),
            ],
            plugboard: Plugboard {
                wire_pairs: config.plugboard,
            },
        }
    }
    pub fn encode(&mut self, input: char) -> char {
        self.advance_rotors();

        let plugboard_result = self.plugboard.encode(input);
        let wheel1_result = self.rotors[0].encode(alphabetical_index(&plugboard_result));
        let wheel2_result = self.rotors[1].encode(wheel1_result);
        let wheel3_result = self.rotors[2].encode(wheel2_result);
        let reflector_result = self.reflect(wheel3_result);
        let wheel3_backwards_result = self.rotors[2].encode_backwards(reflector_result);
        let wheel2_backwards_result = self.rotors[1].encode_backwards(wheel3_backwards_result);
        let wheel1_backwards_result = self.rotors[0].encode_backwards(wheel2_backwards_result);
        self.plugboard
            .encode(alphabetical_letter(wheel1_backwards_result))
    }
    fn advance_rotors(&mut self) {
        let right_rotor_is_on_notch = self.rotors[0].is_on_notch();
        let middle_rotor_is_on_notch = self.rotors[1].is_on_notch();

        self.rotors[0].rotate();
        if middle_rotor_is_on_notch || right_rotor_is_on_notch {
            self.rotors[1].rotate();
        }
        if middle_rotor_is_on_notch {
            self.rotors[2].rotate();
        }
    }
    fn reflect(&self, input: RotorPosition) -> RotorPosition {
        alphabetical_index(&REFLECTOR_B[input])
    }
}

struct Plugboard {
    pub wire_pairs: Vec<[char; 2]>,
}

impl Plugboard {
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
