// Import necessary modules from the standard library
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

// Define a struct to represent a to-do item
#[derive(Debug)]
struct TodoItem {
    name: String,
    completed: bool,        
}

// Function to create a new to-do item
fn create_todo_item(name: String) -> TodoItem {
    // Return a new TodoItem with the given name and a default completed status of false
    TodoItem {
        name, 
        completed: false,
    }
}

// Main function to run the program
fn main() -> io::Result<()> {
    // Define the path to the to-do list file
    let path = Path::new("todos.txt");

    // Attempt to open the file
    let file = File::open(&path);

    // Initialize an empty vector to hold the to-do items
    let mut todo_list: Vec<TodoItem> = Vec::new();

    // If the file was opened successfully, read the to-do items from it
    if let Ok(file) = file {
        for line in BufReader::new(file).lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let completed = parts[0] == "x";
                let name = parts[1].to_string();
                todo_list.push(TodoItem { name, completed });
            }
        }
    }

    // Main loop to interact with the user
    loop {
        // Display the to-do list
        for (i, item) in todo_list.iter().enumerate() {
            println!("{}: {} - {}", i, item.name, if item.completed { "Completed" } else { "Pending" });
        }

        // Ask for a new to-do item or command
        println!("Please enter a new to-do item or command (new or complete):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Split the input into parts
        let parts: Vec<&str> = input.trim().split(' ').collect();

        // Handle the input based on the first part
        match parts[0] {
            "new" => {
                if parts.len() < 2 {
                    println!("Please enter a task name.");
                } else {
                    let name = parts[1].to_string();
                    let item = create_todo_item(name);
                    todo_list.push(item);
                }
            },
            "complete" => {
                match parts[1].parse::<usize>() {
                    Ok(index) => {
                        if index < todo_list.len() {
                            todo_list[index].completed = true;
                        } else {
                            println!("invalid index: {}", index);
                        }
                    },
                    Err(_) => {
                        println!("Please enter a valid index.");
                    },
                }
            },
            _ => {},
        }

        // Save the to-do list to the file
        let mut file = File::create(&path)?;
        for item in &todo_list {
            writeln!(file, "{} {}", if item.completed { "x" } else { " " }, item.name)?;
        }
    }
}
