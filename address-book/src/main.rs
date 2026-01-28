use std::io;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Contact {
    name: String,
    address: String,
    // phone: String,
    // email: String,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.address)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Address Book");
    let mut address_book: Vec<Contact> = load_contacts()?;

    loop {
        println!("\nAvailable Actions: create (c), read (r), update (u), delete (d), quit (q)");

        let mut action = String::new();
        io::stdin().read_line(&mut action)?;

        match action.trim() {
            "create" | "c" => {
                println!("Create a new contact");
                create_contact(&mut address_book)?;
                save_contacts(&address_book)?;
                println!("Contact saved!");
            }
            "read" | "r" => {
                println!("Reading contacts...");
                read_contacts(&address_book);
            }
            "update" | "u" => {
                println!("Update a contact");
                update_contact(&mut address_book)?;
                save_contacts(&address_book)?;
                println!("Contact updated!");
            }
            "delete" | "d" => {
                println!("Delete a contact");
                delete_contact(&mut address_book)?;
                save_contacts(&address_book)?;
            }
            "quit" | "q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Please use a valid action")
        }
    }

    Ok(())
}

// TODO: Write these functions below:
fn create_contact(address_book: &mut Vec<Contact>) -> std::io::Result<()> {
    println!("Name of contact?");
    let mut name_str = String::new();
    io::stdin().read_line(&mut name_str)?;


    println!("Address of contact?");
    let mut address_str = String::new();
    io::stdin().read_line(&mut address_str)?;


    let new_contact = Contact {
        name: name_str.trim().to_string(),
        address: address_str.trim().to_string()
    };

    address_book.push(new_contact);
    Ok(())

}
fn read_contacts(address_book: &Vec<Contact>) {
    if address_book.is_empty() {
        println!("No contacts found!");
        return;
    }

    for (index, contact) in address_book.iter().enumerate() {
        println!("\nContact #{}", index + 1);
        println!("{}\n {}", contact.name, contact.address);
    }
}
fn update_contact(address_book: &mut Vec<Contact>) -> std::io::Result<()> {
    println!("Which contact would you like updated?");

    for (index, contact) in address_book.iter().enumerate() {
        println!("[{}]: {}", index + 1, contact.name);
    }

    let choice:usize = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= address_book.len() => {
                break num;
            }
            _ => println!("Invalid selection")
        }
    };

    let index = choice - 1;

    println!("Updating {}", address_book[index]);

    println!("Enter in new name or if there is no change just hit Enter");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    if !name.trim().is_empty() {
        address_book[index].name = name.trim().to_string();
    }

    println!("Enter in new address or if there is no change just hit Enter");
    let mut address = String::new();
    io::stdin().read_line(&mut address)?;
    if !address.trim().is_empty() {
        address_book[index].address = address.trim().to_string();
    }

    Ok(())
}
fn delete_contact(address_book: &mut Vec<Contact>) -> std::io::Result<()> {

    loop {
        if address_book.is_empty() {
            println!("No contacts found!");
            return Ok(());
        }

        println!("Which contact would you like deleted?");

        for (index, contact) in address_book.iter().enumerate() {
            println!("[{}]: {}", index + 1, contact.name);
        }

        let choice:usize = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<usize>() {
                Ok(num) if num >= 1 && num <= address_book.len() => {
                    break num;
                }
                _ => println!("Invalid selection")
            }
        };

        let index = choice - 1;

        println!("Are you sure you'd like to delete [{}]? (Y/n)", address_book[index]);

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "n" => {
                break;
            }
            _ => {
                println!("Deleting [{}]", address_book[index]);
                address_book.remove(index);
                println!("Deleted contact");
                break;
            }
            // _ => {
            //     println!("Invalid action");
            //     continue;
            // }
        }
    }
    Ok(())
}
fn save_contacts(contacts: &Vec<Contact>) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path("contacts.csv")?;
    for contact in contacts {
        wtr.serialize(contact)?;
    }

    wtr.flush()?;
    Ok(())
}
fn load_contacts() -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let mut contacts = Vec::new();

    if std::path::Path::new("contacts.csv").exists() {
        let mut rdr = Reader::from_path("contacts.csv")?;

        for result in rdr.deserialize() {
            let record: Contact = result?;
            contacts.push(record);
        }
    }

    Ok(contacts)
}
