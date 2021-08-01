use enigma_machine::{Config, EnigmaMachine};

fn main() {
    let config = load_config();
    let plain_text = get_plain_text();
    let cipher_text = encode(&plain_text, config);

    println!("Plain text: {}", plain_text);
    println!("Cipher text: {}", cipher_text);
}

fn load_config() -> Config {
    let file = std::fs::File::open("config.yaml").unwrap();
    let config: Config = serde_yaml::from_reader(file).unwrap();
    if let Err(text) = config.verify() {
        panic!("{}", text);
    }
    config
}

fn get_plain_text() -> String {
    let mut plain_text = String::new();
    std::io::stdin().read_line(&mut plain_text).unwrap();
    plain_text.strip_suffix("\n").unwrap().to_string()
}

fn encode(plain_text: &str, config: Config) -> String {
    let mut enigma_machine = EnigmaMachine::new(config);
    plain_text
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            enigma_machine.encode(c.to_ascii_uppercase())
        })
        .collect()
}
