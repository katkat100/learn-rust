use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("sample.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_word_count: u32 = 0;
    let words = contents.split_whitespace();
    for word in words {
        total_word_count += 1;
    }

    println!("Total word count: {}", total_word_count);

    Ok(())
}
