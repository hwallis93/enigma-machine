use crate::{
    config::RotorConfig,
    constants::{ALPHABET, ROTORS},
    utils::{alphabetical_index, alphabetical_letter},
};

/// Every contact on a rotor can be in one of 26 positions
///
/// A rotor's input and output are considered in terms of these static positions rather than
/// letters, which will change position as the rotor rotates.
///
/// Uses `usize` instead of e.g. `i32` as it is used to index into arrays
pub type RotorPosition = usize;

/// A rotor sitting in a slot
pub struct Rotor {
    /// The letters, in order, that A, B, C etc are wired to. Accounts for ring setting
    pub wiring: Vec<char>,

    /// The letters with notches next to them
    pub notches: [char; 2],

    /// How far the rotor has rotated in its slot. An offset of 0 indicates that 'A' is showing in
    /// the display window
    pub offset: usize,
}

impl Rotor {
    pub fn new(config: RotorConfig) -> Self {
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

    /// Rotate the rotor around 1 position
    pub fn rotate(&mut self) {
        self.offset = (self.offset + 1) % 26
    }

    /// Encode a signal through the rotor from right to left
    pub fn encode(&self, input: RotorPosition) -> RotorPosition {
        let input_letter_alphabetical_index = (input + self.offset) % 26;
        let output_letter = self.wiring[input_letter_alphabetical_index];

        (alphabetical_index(&output_letter) + 26 - self.offset) % 26
    }

    /// Encode a signal through the rotor from left to right
    pub fn encode_backwards(&self, input: RotorPosition) -> RotorPosition {
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
    pub fn is_on_notch(&self) -> bool {
        self.notches.contains(&alphabetical_letter(self.offset))
    }
}
