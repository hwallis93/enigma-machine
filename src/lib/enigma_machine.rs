use super::config::Config;
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
    /// The letters, in order, that A, B, C etc are wired to when the rotor is at the default
    /// setting (of 0)
    pub wiring: [char; 26],

    /// The letter which the notch is at, hardcoded into the rotor (TODO)
    pub notches: [char; 2],

    /// How far the wiring is rotated away from the default position
    pub setting: usize,

    /// How far the rotor has rotated in its slot. An offset of 0 indicates that 'A' is showing in
    /// the display window
    pub offset: usize,
}
struct Plugboard {
    pub wire_pairs: Vec<[char; 2]>,
}

impl Rotor {
    fn rotate(&mut self) {
        self.offset = (self.offset + 1) % 26
    }
    fn encode(&self, input: RotorPosition) -> RotorPosition {
        let input_letter_alphabetical_index = (input + 26 + self.offset - self.setting) % 26;
        let output_letter = self.wiring[input_letter_alphabetical_index];

        (alphabetical_index(output_letter) + 26 - self.offset + self.setting) % 26
    }

    fn encode_backwards(&self, input: RotorPosition) -> RotorPosition {
        let input_letter = alphabetical_letter((input + self.offset + 26 - self.setting) % 26);
        let output_letter = alphabetical_letter(
            self.wiring
                .iter()
                .position(|c| *c == input_letter)
                .expect("Received unknown character")
                % ALPHABET.len(),
        );

        (alphabetical_index(output_letter) + 26 - self.offset + self.setting) % 26
    }

    /// Whether the next rotation of this rotor will cause a nieghbouring rotor to rotate too
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
        let rotors = [
            Rotor {
                wiring: ROTORS[config.right_rotor.number - 1].wiring,
                notches: ROTORS[config.right_rotor.number - 1].notches,
                setting: 0,
                offset: 0,
            },
            Rotor {
                wiring: ROTORS[config.middle_rotor.number - 1].wiring,
                notches: ROTORS[config.middle_rotor.number - 1].notches,
                setting: 0,
                offset: 0,
            },
            Rotor {
                wiring: ROTORS[config.left_rotor.number - 1].wiring,
                notches: ROTORS[config.left_rotor.number - 1].notches,
                setting: 0,
                offset: 0,
            },
        ];
        let plugboard = Plugboard {
            wire_pairs: config.plugboard,
        };

        EnigmaMachine { rotors, plugboard }
    }
    pub fn encode(&mut self, input: char) -> char {
        self.advance_rotors();

        let plugboard_result = self.plugboard.encode(input);
        let wheel1_result = self.rotors[0].encode(alphabetical_index(plugboard_result));
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
        if self.rotors[0].is_on_notch() {
            if self.rotors[1].is_on_notch() {
                self.rotors[2].rotate()
            }
            self.rotors[1].rotate()
        }
        self.rotors[0].rotate()
    }
    fn reflect(&self, input: RotorPosition) -> RotorPosition {
        alphabetical_index(REFLECTOR_B[input])
    }
}

fn alphabetical_index(letter: char) -> usize {
    ALPHABET
        .iter()
        .position(|c| *c == letter)
        .expect("Received unknown character")
}
fn alphabetical_letter(index: usize) -> char {
    ALPHABET[index % ALPHABET.len()]
}
