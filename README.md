# Enigma Machine

This crate provides:
  - A library for encoding letters via an enigma machine in a specified setup
  - A simple command line interface that can read config from a file and encode user input

## Library

The library provides:
  - `EnigmaMachine`, representing the machine itself
  - `Config` and `RotorConfig`, representing the machine's settings
  - `ConfigError`, an enum of possible errors returned due to invalid config

Usage:
```rust
use enigma_machine::{Config, RotorConfig, EnigmaMachine};

// Build a `Config` struct
let config = Config {
    left_rotor: RotorConfig {
        number: 1,
        setting: 5,
        window_letter: 'D'
    },
    middle_rotor: RotorConfig {
        number: 3,
        setting: 9,
        window_letter: 'R'
    },
    right_rotor: RotorConfig {
        number: 7,
        setting: 24,
        window_letter: 'U'
    },
    plugboard: vec![['A', 'R'], ['E', 'M']]
};

// Create an `EngimaMachine` using the `Config`
match EnigmaMachine::new(config) {
    Ok(mut machine) => {
        // Encode characters using the machine. Note it only accepts 'A'..'Z'
        let cipher_text: String = "HELLO".chars().map(|letter| machine.encode(letter)).collect();
    }
    Err(errors) => {
        // Handle errors
    }
}
```

## Command line interface
Create a file called `config.yaml`, see [config.template.yaml](./config.template.yaml) for syntax.

Then `cargo run`, type your plaintext and hit Enter

## TODO
- Publish library
- Publish binaries
  - Docker?
- Configurable reflector

