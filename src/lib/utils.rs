use crate::constants::ALPHABET;

pub fn alphabetical_index(letter: &char) -> usize {
    ALPHABET
        .iter()
        .position(|c| c == letter)
        .expect("Received unknown character")
}
pub fn alphabetical_letter(index: usize) -> char {
    ALPHABET[index % ALPHABET.len()]
}
