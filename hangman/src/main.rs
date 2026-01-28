use std::io;
use std::collections::HashSet;
use rand::Rng;

const MAX_GUESSES: usize = 6;

fn main() {
    const HANGMAN: [&str; 7] = [
    r#"
      +---+
      |   |
          |
          |
          |
          |
    ========="#, r#"
      +---+
      |   |
      O   |
          |
          |
          |
    ========="#, r#"
      +---+
      |   |
      O   |
      |   |
          |
          |
    ========="#, r#"
      +---+
      |   |
      O   |
     /|   |
          |
          |
    ========="#, r#"
      +---+
      |   |
      O   |
     /|\  |
          |
          |
    ========="#, r#"
      +---+
      |   |
      O   |
     /|\  |
     /    |
          |
    ========="#, r#"
      +---+
      |   |
      O   |
     /|\  |
     / \  |
          |
    ========="#];
    const WORDS: [&str; 30] = [
        "cat", "dog", "elephant", "tiger", "lion",
        "computer", "keyboard", "mouse", "monitor", "laptop",
        "ocean", "mountain", "forest", "desert", "river",
        "guitar", "piano", "drums", "violin", "trumpet",
        "pizza", "burger", "pasta", "sushi", "tacos",
        "summer", "winter", "spring", "autumn", "season"
    ];

    loop {
        let secret_number: usize = rand::thread_rng().gen_range(0..=WORDS.len());

        let secret_word: &str = WORDS[secret_number];

        let mut guesses: HashSet<char> = HashSet::new();
        let mut remaining_guesses = 0;

        println!("Welcome to Hangman!");

        loop {
            println!("\n{}", HANGMAN[remaining_guesses]);
            print!("Word: ");
            for ch in secret_word.chars() {
                if guesses.contains(&ch) {
                    print!("{} ", ch);
                } else {
                    print!("_ ");
                }
            }
            println!("");

            println!("Guess a letter:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let guess_str = input.trim();
            if guess_str.len() != 1 {
                println!("Please enter a single letter!");
                continue;
            }

            let guess_char = guess_str.chars().next().unwrap();
            if guesses.contains(&guess_char) {
                println!("You already guessed that letter!");
                continue;
            }

            guesses.insert(guess_char);

            println!("Guesses: {:?}", guesses);

            // Check against secret word
            if secret_word.contains(guess_char) {
                // If correct update to show what is left of secret word
                println!("Correct!");

                // check if got whole word if did send win! and exit
                if secret_word.chars().all(|ch| guesses.contains(&ch)) {
                    println!("You win! The word was: {}", secret_word);
                    break;
                }
            } else {
                // If wrong tell them wrong and minus 1 from remaining_guesses
                println!("Wrong!");
                remaining_guesses += 1;
                // check if out of guesses if so send lose! and exit
                if remaining_guesses == MAX_GUESSES {
                    println!("\n{}", HANGMAN[remaining_guesses]);
                    println!("You lose! The word was: {}", secret_word);
                    break;
                }
            }
        }

        println!("Would you like to play again? Y/n");

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");

        match action.trim() {
            "n" => break,
            _ => continue,
        }
    }
}
