use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;

fn main() -> std::io::Result<()> {
    println!("Todo List");
    // Create list if not created
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todo.text")?;

    // Prompt user for action
    println!("Available actions: create (c), read (r), update (u), delete (d)");
    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("Failed to read line");
    match action.trim() {
        "create" | "c" => {
            println!("Create new item:");
            let mut todo = String::new();
            io::stdin().read_line(&mut todo).expect("Failed to read line");
            file.write_all(todo.as_bytes()).expect("Failed to write to file");
        }
        "read" | "r" => {
            let mut file = File::open("todo.text")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("Todo list:");
            println!("{}", contents);
        }
        "update" | "u" => {
            println!("Update which item:");
            let mut file = File::open("todo.text")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let lines: Vec<&str> = contents.lines().collect();
            display_todo(&lines);

            let choice: usize = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                match input.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= lines.len() => {
                        break num;
                    }
                    _ => println!("Invalid item")
                }
            };

            let index = choice - 1;

            println!("Current item: {}", lines[index]);
            println!("Enter new item:");

            let mut new_text = String::new();
            io::stdin().read_line(&mut new_text).expect("Failed to read line");

            println!("Updated item...");
            let mut updated_lines:Vec<String> = lines.iter().map(|line| line.to_string()).collect();
            updated_lines[index] = new_text.trim().to_string();

            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)  // This clears the file
                .open("todo.text")?;

            for line in updated_lines {
                writeln!(file, "{}", line)?;
            }
        }
        "delete" | "d" => {
            println!("Delete Item...");
            let mut file = File::open("todo.text")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let lines: Vec<&str> = contents.lines().collect();
            display_todo(&lines);

            let choice: usize = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                match input.trim().parse::<usize>() {
                    Ok(num) if num >= 1 && num <= lines.len() => {
                        break num;
                    }
                    _ => println!("Invalid item")
                }
            };

            let index = choice - 1;

            println!("Current item: {}", lines[index]);
            println!("Are you sure you want to delete item? (y/n)");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)  // This clears the file
                        .open("todo.text")?;

                    let mut updated_lines:Vec<String> = lines.iter().map(|line| line.to_string()).collect();
                    updated_lines.remove(index);

                    for line in updated_lines {
                        writeln!(file, "{}", line)?;
                    }
                    println!("Item deleted");
                }
                _ => println!("Item not deleted")
            }
        }
        _ => println!("Invalid action"),
    }

    Ok(())
}

fn display_todo(lines: &[&str]) {
    println!("Todo list:");
    for (index, line) in lines.iter().enumerate() {
        println!("{}. {}", index + 1, line);
    }
}
