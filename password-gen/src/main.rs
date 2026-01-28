use rand::Rng;

const PASSWORD_LENGTH: usize = 12;
const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;':\"/<>?";

fn main() {
    let mut gen_password = String::new();
    let character_char: Vec<char> = CHARACTERS.chars().collect();
    for _ in 0..PASSWORD_LENGTH {
        let index = rand::thread_rng().gen_range(0..character_char.len());
        gen_password.push(character_char[index]);
    }
    println!("{}", gen_password);
}
