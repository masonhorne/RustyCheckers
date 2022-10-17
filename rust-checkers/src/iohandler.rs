use std::io;

pub fn read_int(prompt: String) -> i32 {
    // Loop until valid input is provided
    loop {
        // Initialize string to hold input
        let mut input_text = String::new();
        // Output the provided prompt
        println!("{}", prompt);
        // Read line from standard input
        io::stdin()
            .read_line(&mut input_text)
            .expect("No input was provided from stdin.");
        // Trim the line
        let trimmed = input_text.trim();
        // Parse the integer and return if valid otherwise prompt again.
        match trimmed.parse::<i32>() {
            Ok(i) => {
                if i > 0 && i < 9 {
                    return i;
                } else {
                    println!("Invalid integer provided: {}\nPlease try again.\n", trimmed)
                }
            },
            Err(..) => println!("Invalid integer provided: {}\nPlease try again.\n", trimmed),
        };
    }
}