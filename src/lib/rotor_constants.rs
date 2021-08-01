pub struct RotorConstants {
    pub wiring: [char; 26],
    pub notches: [char; 2],
}

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const ROTOR_I: RotorConstants = RotorConstants {
    wiring: [
        'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U',
        'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J',
    ],
    notches: ['Q', '_'],
};
const ROTOR_II: RotorConstants = RotorConstants {
    wiring: [
        'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G',
        'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E',
    ],
    notches: ['E', '_'],
};
const ROTOR_III: RotorConstants = RotorConstants {
    wiring: [
        'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W',
        'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
    ],
    notches: ['V', '_'],
};
const ROTOR_IV: RotorConstants = RotorConstants {
    wiring: [
        'E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F',
        'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B',
    ],
    notches: ['J', '_'],
};
const ROTOR_V: RotorConstants = RotorConstants {
    wiring: [
        'V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A', 'W',
        'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K',
    ],
    notches: ['Z', '_'],
};
const ROTOR_VI: RotorConstants = RotorConstants {
    wiring: [
        'J', 'P', 'G', 'V', 'O', 'U', 'M', 'F', 'Y', 'Q', 'B', 'E', 'N', 'H', 'Z', 'R', 'D', 'K',
        'A', 'S', 'X', 'L', 'I', 'C', 'T', 'W',
    ],
    notches: ['M', 'Z'],
};
const ROTOR_VII: RotorConstants = RotorConstants {
    wiring: [
        'N', 'Z', 'J', 'H', 'G', 'R', 'C', 'X', 'M', 'Y', 'S', 'W', 'B', 'O', 'U', 'F', 'A', 'I',
        'V', 'L', 'P', 'E', 'K', 'Q', 'D', 'T',
    ],
    notches: ['M', 'Z'],
};
const ROTOR_VIII: RotorConstants = RotorConstants {
    wiring: [
        'F', 'K', 'Q', 'H', 'T', 'L', 'X', 'O', 'C', 'B', 'J', 'S', 'P', 'D', 'Z', 'R', 'A', 'M',
        'E', 'W', 'N', 'I', 'U', 'Y', 'G', 'V',
    ],
    notches: ['M', 'Z'],
};
pub const ROTORS: [RotorConstants; 8] = [
    ROTOR_I, ROTOR_II, ROTOR_III, ROTOR_IV, ROTOR_V, ROTOR_VI, ROTOR_VII, ROTOR_VIII,
];

pub const REFLECTOR_B: [char; 26] = [
    'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B', 'F',
    'Z', 'C', 'W', 'V', 'J', 'A', 'T',
];
