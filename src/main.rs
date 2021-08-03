use enigma_machine::EnigmaMachine;

fn main() {
    let file = std::fs::File::open("config.yaml").unwrap_or_else(|err| {
        println!("Error: Could not open config file:\n{:?}", err);
        std::process::exit(1)
    });
    let config = serde_yaml::from_reader(file).unwrap_or_else(|err| {
        println!("Error: could not parse config file:\n{:?}", err);
        std::process::exit(2)
    });
    let mut machine = EnigmaMachine::new(config).unwrap_or_else(|errors| {
        println!("Error: Config file is invalid:\n{:?}", errors);
        std::process::exit(3);
    });

    let mut plain_text = String::new();
    println!("Type plain text and hit <Enter>:");
    std::io::stdin()
        .read_line(&mut plain_text)
        .unwrap_or_else(|err| {
            println!("Error: Failed to read user input:\n{:?}", err);
            std::process::exit(4);
        });

    let cipher_text = encode(&mut machine, &plain_text);
    print!("\nPlain text: {}", plain_text);
    println!("Cipher text: {}", cipher_text);
}

fn encode(machine: &mut EnigmaMachine, plain_text: &str) -> String {
    plain_text
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            machine.encode(c.to_ascii_uppercase())
        })
        .collect()
}
