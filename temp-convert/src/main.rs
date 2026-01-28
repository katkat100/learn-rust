use std::io;

fn main() {
    println!("Convert Temp!");

    loop {
        println!("Convert to Celsius or Fahrenheit? (c/f)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = match choice.trim() {
            "c" => "Celsius",
            "f" => "Fahrenheit",
            _ => {
                println!("Invalid choice");
                continue;
            },
        };

        let opposite = match choice {
            "Celsius" => "Fahrenheit",
            "Fahrenheit" => "Celsius",
            _ => "Invalid choice",
        };

        loop {
            println!("What {} temp should we convert?", opposite);

            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Failed to read line");

            let temp: f64 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input");
                    continue;
                }
            };

            println!("Converting: {} {}", temp, opposite);

            let converted = match choice {
                "Celsius" => (temp - 32.0) / 1.8,
                "Fahrenheit" => (temp * 1.8) + 32.0,
                _ => 0.0,
            };

            println!("Temperature: {} {}", converted, choice);
            break;
        }

        break;
    }
}
