use std::fmt::Debug;

pub struct EnigmaMachine {
    rotors: [Rotor; 3],
    plugboard: Plugboard,
}

#[derive(Debug)]
pub struct Rotor {
    // The letters, in order, that A, B, C etc will encode into when the rotor has no offset (i.e.
    // is displaying "A")
    pub wiring: [char; 26],

    // The letter displayed by the rotor indicating how far it has rotated and hence the offset of
    // its internal wiring from the external wiring
    pub offset: char,
}
pub struct Plugboard {
    pub wire_pairs: Vec<(char, char)>,
}

impl Rotor {
    fn rotate(&mut self) {
        let new_index = alphabetical_index(self.offset) + 1;

        self.offset = match new_index.cmp(&27) {
            std::cmp::Ordering::Less => alphabetical_letter(new_index),
            std::cmp::Ordering::Equal => 'A',
            std::cmp::Ordering::Greater => {
                panic!("Calculated invalid rotation index {}", new_index)
            }
        }
    }
    fn encode(&self, input: char) -> char {
        let index = (alphabetical_index(input) + alphabetical_index(self.offset)) % ALPHABET.len();
        println!("Encode index {:#?}", index);
        let res = self.wiring[index];
        println!("Found {:#?}", res);
        res
        // self.wiring[(alphabetical_index(input) + alphabetical_index(self.offset)) % ALPHABET.len()]
    }
    fn encode_backwards(&self, input: char) -> char {
        alphabetical_letter(
            (self
                .wiring
                .iter()
                .position(|c| *c == input)
                .expect("Received unknown character")
                + ALPHABET.len()
                - alphabetical_index(self.offset))
                % ALPHABET.len(),
        )
    }
}

impl Plugboard {
    fn encode(&self, input: char) -> char {
        for (letter1, letter2) in self.wire_pairs.iter() {
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
    pub fn new(rotors: [Rotor; 3], plugboard: Plugboard) -> Self {
        EnigmaMachine { rotors, plugboard }
    }
    pub fn encode(&mut self, input: char) -> char {
        self.advance_rotors();
        // TODO - Issue is that the outbound index of a rotor is also affected by the offset, so
        // if an encode emits 'F' (5) but has offset 'C' (2) then need to start the encode at the next
        // rotor's 3 position, which is 'D' if it has no offset
        // https://piotte13.github.io/enigma-cipher/

        let plugboard_result = self.plugboard.encode(input);
        println!("{:#?}", plugboard_result);
        let wheel1_result = self.rotors[0].encode(plugboard_result);
        println!("{:#?}", wheel1_result);
        let wheel2_result = self.rotors[1].encode(wheel1_result);
        println!("{:#?}", wheel2_result);
        let wheel3_result = self.rotors[2].encode(wheel2_result);
        println!("{:#?}", wheel3_result);
        let reflector_result = self.reflect(wheel3_result);
        println!("{:#?}", reflector_result);
        let wheel3_backwards_result = self.rotors[2].encode_backwards(reflector_result);
        println!("{:#?}", wheel3_backwards_result);
        let wheel2_backwards_result = self.rotors[1].encode_backwards(wheel3_backwards_result);
        println!("{:#?}", wheel2_backwards_result);
        let wheel1_backwards_result = self.rotors[0].encode_backwards(wheel2_backwards_result);
        println!("{:#?}", wheel1_backwards_result);
        let plugboard_final_result = self.plugboard.encode(wheel1_backwards_result);
        println!("{:#?}", plugboard_final_result);

        plugboard_final_result
    }
    fn advance_rotors(&mut self) {
        self.rotors[0].rotate();
        if self.rotors[0].offset == 'A' {
            self.rotors[1].rotate();

            if self.rotors[1].offset == 'A' {
                self.rotors[2].rotate()
            }
        }
    }
    fn reflect(&self, input: char) -> char {
        REFLECTOR_B[alphabetical_index(input)]
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

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const ROTOR_I_WIRING: [char; 26] = [
    'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S',
    'P', 'A', 'I', 'B', 'R', 'C', 'J',
];
pub const ROTOR_II_WIRING: [char; 26] = [
    'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G', 'Z',
    'N', 'P', 'Y', 'F', 'V', 'O', 'E',
];
pub const ROTOR_III_WIRING: [char; 26] = [
    'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W', 'G',
    'A', 'K', 'M', 'U', 'S', 'Q', 'O',
];
pub const ROTOR_IV_WIRING: [char; 26] = [
    'E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F', 'T',
    'G', 'K', 'D', 'C', 'M', 'W', 'B',
];
pub const ROTOR_V_WIRING: [char; 26] = [
    'V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W', 'M',
    'J', 'Q', 'O', 'F', 'E', 'C', 'K',
];
pub const ROTOR_VI_WIRING: [char; 26] = [
    'J', 'P', 'G', 'V', 'O', 'U', 'M', 'F', 'Y', 'Q', 'B', 'E', 'N', 'H', 'Z', 'R', 'D', 'K', 'A',
    'S', 'X', 'L', 'I', 'C', 'T', 'W',
];
pub const ROTOR_VII_WIRING: [char; 26] = [
    'N', 'Z', 'J', 'H', 'G', 'R', 'C', 'X', 'M', 'Y', 'S', 'W', 'B', 'O', 'U', 'F', 'A', 'I', 'V',
    'L', 'P', 'E', 'K', 'Q', 'D', 'T',
];
pub const ROTOR_VIII_WIRING: [char; 26] = [
    'F', 'K', 'Q', 'H', 'T', 'L', 'X', 'O', 'C', 'B', 'J', 'S', 'P', 'D', 'Z', 'R', 'A', 'M', 'E',
    'W', 'N', 'I', 'U', 'Y', 'G', 'V',
];
pub const ROTOR_WIRINGS: [[char; 26]; 8] = [
    ROTOR_I_WIRING,
    ROTOR_II_WIRING,
    ROTOR_III_WIRING,
    ROTOR_IV_WIRING,
    ROTOR_V_WIRING,
    ROTOR_VI_WIRING,
    ROTOR_VII_WIRING,
    ROTOR_VIII_WIRING,
];

const REFLECTOR_B: [char; 26] = [
    'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B', 'F',
    'Z', 'C', 'W', 'V', 'J', 'A', 'T',
];
