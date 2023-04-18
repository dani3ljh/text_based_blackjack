use std::io::{self, Write};

fn integer_between_values(min: i32, max: i32) -> i32 {
    let mut input = String::new();
    loop {
        print!("Please enter an integer between {} and {}: ", min, max);
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) if num >= min && num <= max => return num,
            _ => println!("Invalid input. Please enter an integer between {} and {}.", min, max),
        }

        input.clear();
    }
}

fn integer_with_values(values: Vec<i32>) -> i32 {
	let mut input = String::new();
	loop {
		print!("Please enter an integer from the following list: ");
		for value in &values {
			print!("{}, ", value);
		}
		println!();
		io::stdout().flush().expect("Failed to flush stdout");

		io::stdin().read_line(&mut input).expect("Failed to read line");

		match input.trim().parse::<i32>() {
			Ok(num) if values.contains(&num) => return num,
			_ => println!("Invalid input. Please enter an integer from the following list:"),
		}

		input.clear();
	}
}

fn string_with_values(values: Vec<String>) -> String {
	let mut input = String::new();
	loop {
		print!("Please enter a string from the following list: ");
		for value in &values {
			print!("{}, ", value);
		}
		println!();
		io::stdout().flush().expect("Failed to flush stdout");

		io::stdin().read_line(&mut input).expect("Failed to read line");

		match input.trim().parse::<String>() {
			Ok(string) if values.contains(&string) => return string,
			_ => println!("Invalid input. Please enter a string from the following list:"),
		}

		input.clear();
	}
}