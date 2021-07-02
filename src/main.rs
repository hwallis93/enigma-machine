mod rotor;
use crate::rotor::*;

// TODO serde
struct Config {
    left_rotor: usize,
    middle_rotor: usize,
    right_rotor: usize,
    plugboard: Vec<(char, char)>,
}
// TODO verification. Can be done with serde?

fn main() {
    // let config = load_config()
    let config = Config {
        left_rotor: 0,
        middle_rotor: 1,
        right_rotor: 2,
        plugboard: vec![],
    };
    let left_rotor = Rotor {
        wiring: ROTOR_WIRINGS[config.left_rotor],
        offset: 'A',
    };
    let middle_rotor = Rotor {
        wiring: ROTOR_WIRINGS[config.middle_rotor],
        offset: 'A',
    };
    let right_rotor = Rotor {
        wiring: ROTOR_WIRINGS[config.right_rotor],
        offset: 'A',
    };
    let mut enigma_machine = EnigmaMachine::new(
        // Encoding travels from right to left
        [right_rotor, middle_rotor, left_rotor],
        Plugboard {
            wire_pairs: config.plugboard,
        },
    );
    let plain_text = ['A'];
    println!("{:#?}", plain_text);

    let cipher_text: String = plain_text
        .iter()
        .map(|c| enigma_machine.encode(*c))
        .collect();
    println!("{:?}", cipher_text);
}
