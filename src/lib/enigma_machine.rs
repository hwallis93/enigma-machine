use super::config::{Config, RotorConfig};
use super::rotor_constants::{ALPHABET, REFLECTOR_B, ROTORS};

/// Every contact on a rotor can be in one of 26 positions
///
/// A rotor's input and output are considered in terms of these static positions rather than
/// letters, which will change position as the rotor rotates.
///
/// Uses `usize` instead of e.g. `i32` as it is used to index into arrays
type RotorPosition = usize;
pub struct EnigmaMachine {
    rotors: [Rotor; 3],
    plugboard: Plugboard,
}

/// A rotor sitting in a slot
struct Rotor {
    /// The letters, in order, that A, B, C etc are wired to. Accounts for ring setting
    pub wiring: Vec<char>,

    /// The letter which the notch is at, hardcoded into the rotor
    pub notches: [char; 2],

    /// How far the rotor has rotated in its slot. An offset of 0 indicates that 'A' is showing in
    /// the display window
    pub offset: usize,
}
struct Plugboard {
    pub wire_pairs: Vec<[char; 2]>,
}

impl Rotor {
    fn new(config: RotorConfig) -> Self {
        let wiring = ROTORS[config.number - 1].wiring;
        let rotated_wiring = [
            &wiring[26 - config.setting..],
            &wiring[..26 - config.setting],
        ]
        .concat()
        .iter()
        .map(|letter| alphabetical_letter(alphabetical_index(letter) + config.setting))
        .collect();

        Rotor {
            wiring: rotated_wiring,
            notches: ROTORS[config.number - 1].notches,
            offset: alphabetical_index(&config.window_letter),
        }
    }

    /// Rotate the rotor by 1 position
    fn rotate(&mut self) {
        self.offset = (self.offset + 1) % 26
    }

    /// Encode a signal through the rotor from right to left
    fn encode(&self, input: RotorPosition) -> RotorPosition {
        let input_letter_alphabetical_index = (input + self.offset) % 26;
        let output_letter = self.wiring[input_letter_alphabetical_index];

        (alphabetical_index(&output_letter) + 26 - self.offset) % 26
    }

    /// Encode a signal through the rotor from left to right
    fn encode_backwards(&self, input: RotorPosition) -> RotorPosition {
        let input_letter = alphabetical_letter((input + self.offset) % 26);
        let output_letter = alphabetical_letter(
            self.wiring
                .iter()
                .position(|c| *c == input_letter)
                .expect("Received unknown character")
                % ALPHABET.len(),
        );

        (alphabetical_index(&output_letter) + 26 - self.offset) % 26
    }

    /// Whether a keystroke will engage one of this rotor's notches
    fn is_on_notch(&self) -> bool {
        self.notches.contains(&alphabetical_letter(self.offset))
    }
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

fn alphabetical_index(letter: &char) -> usize {
    ALPHABET
        .iter()
        .position(|c| c == letter)
        .expect("Received unknown character")
}
fn alphabetical_letter(index: usize) -> char {
    ALPHABET[index % ALPHABET.len()]
}
