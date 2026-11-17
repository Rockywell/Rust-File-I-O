use std::fs;
use std::io::{self, Write};

// Runs the main interface loop for the user to interact with the file manager program.
fn main() -> io::Result<()> {
    let filename = "text.txt";
    let mut lines = load_file(filename)?;

    loop {
        println!("\n=== Text File Manager ===");
        println!("1. View text");
        println!("2. Add line");
        println!("3. Edit line");
        println!("4. Delete line");
        println!("5. Save file");
        println!("6. Exit");

        let choice = get_input("Choose an option: ")?;

        match choice.as_str() {
            "1" => show_lines(&lines),
            "2" => add_line(&mut lines)?,
            "3" => edit_line(&mut lines)?,
            "4" => delete_line(&mut lines)?,
            "5" => {
                // Saves the vector and writes it to the file.
                fs::write(filename, lines.join("\n"))?;
                println!("File saved.");
            }
            "6" => break,
            _ => println!("Invalid option."),
        }
    }

    Ok(())
}

// ===Helper Functions===
// Read a line entered by the user
fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;

    // Return with an error if the input stream was closed and no bytes were read.
    if bytes_read == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Input was closed",
        ));
    }

    // Formats the input properly, trims the end and removes newline characters.
    Ok(input.trim_end_matches(&['\r', '\n'][..]).to_string())
}

// ===I/O File Functions===
// Load text from the file into a vector
fn load_file(filename: &str) -> io::Result<Vec<String>> {
    // Reads from the selected file and returns its contents, if empty returns an empty string.
    let contents = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error),
    };

    let mut lines = Vec::new();

    for line in contents.lines() {
        lines.push(line.to_string());
    }

    Ok(lines)
}

// Display all lines with their numbers
fn show_lines(lines: &[String]) {
    if lines.is_empty() {
        println!("The file is empty.");
        return;
    }

    for (index, line) in lines.iter().enumerate() {
        println!("{}: {}", index + 1, line);
    }
}

// Add a line to the vector
fn add_line(lines: &mut Vec<String>) -> io::Result<()> {
    let text = get_input("Enter new text: ")?;
    lines.push(text);

    println!("Line added.");
    Ok(())
}

// Choose a line to edit or delete by returning its selected index
fn select_line(lines: &[String]) -> io::Result<Option<usize>> {
    if lines.is_empty() {
        println!("There are no lines to select.");
        return Ok(None);
    }

    show_lines(lines);

    let input = get_input("Enter line number: ")?;
    let number = input.trim().parse::<usize>();

    // Extract the number from okay if the result is a successful value.
    if let Ok(number) = number {
        if number > 0 && number <= lines.len() {
            return Ok(Some(number - 1));
        }
    }

    println!("Invalid line number.");
    Ok(None)
}

// Replace one line in the vector
fn edit_line(lines: &mut Vec<String>) -> io::Result<()> {
    //Extract the result to the index option which is then extracted to the index if it's valid and exists.
    if let Some(index) = select_line(lines)? {
        let text = get_input("Enter replacement text: ")?;
        lines[index] = text;
        println!("Line updated.");
    }

    Ok(())
}

// Remove one line from the vector
fn delete_line(lines: &mut Vec<String>) -> io::Result<()> {
    //Extract the result to the index option which is then extracted to the index if it's valid and exists.
    if let Some(index) = select_line(lines)? {
        lines.remove(index);
        println!("Line deleted.");
    }

    Ok(())
}
