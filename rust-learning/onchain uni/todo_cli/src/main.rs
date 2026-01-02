// Import necessary libraries
use clap::{App, Arg};  // For command line argument parsing
use serde::{Deserialize, Serialize};  // For JSON serialization/deserialization
use std::error::Error; // For error handling

// Define task structure with JSON capabilities
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,           // Unique identifier for each task
    content: String,   // Actual task text
    completed: bool,   // Completion status
}

fn main() -> Result<(), Box<dyn Error>> {
    // Configure command line interface using clap
    let matches = App::new("ToDo CLI")
        .version("1.0")
        .author("Author: Pratham Jaiswal")
        .about("Manage your tasks")

        // Add command: -a/--add <TASK>
        .arg(
            Arg::with_name("add")
                .short('a')
                .long("add")
                .value_name("TASK")
                .help("Adds a task to your to-do list")
                .takes_value(true),
        )
        
        // List command: -l/--list
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .help("Lists all tasks")
                .takes_value(false),
        )
        
        // Check command: -c/--check <ID>
        .arg(
            Arg::with_name("check")
                .short('c')
                .long("check")
                .value_name("ID")
                .help("Marks a task as checked by ID")
                .takes_value(true),
        )

        // Uncheck command: -u/--uncheck
        .arg(
            Arg::with_name("uncheck")
            .short('u')
            .long("uncheck")
            .value_name("ID")
            .help("Marks a task as unchecked by ID")
            .takes_value(true),
        )
        
        // Delete command: -d/--delete <ID>
        .arg(
            Arg::with_name("delete")
                .short('d')
                .long("delete")
                .value_name("ID")
                .help("Deletes a task by ID")
                .takes_value(true),
        )
        .get_matches();

    // Load existing tasks from JSON file
    let mut tasks = load_tasks()?;

    // Handle 'add' command
    if let Some(task) = matches.value_of("add") {
        add_task(&mut tasks, task.to_string());
        save_tasks(&tasks)?;
        println!("✅ Task added successfully");
    }

    // Handle 'list' command
    if matches.is_present("list") {
        list_tasks(&tasks);
    }

    // Handle 'check' command
    if let Some(id) = matches.value_of("check") {
        // Convert string input to numeric ID
        let id = id.parse().expect("Invalid task ID");
        if check_task(&mut tasks, id) {
            save_tasks(&tasks)?;
            println!("✅ Task {} marked as complete", id);
        } else {
            println!("❌ Task {} not found", id);
        }
    }

    // Handle 'delete' command
    if let Some(id) = matches.value_of("delete") {
        // Convert string input to numeric ID
        let id = id.parse().expect("Invalid task ID");
        if delete_task(&mut tasks, id) {
            save_tasks(&tasks)?;
            println!("✅ Task {} deleted", id);
        } else {
            println!("❌ Task {} not found", id);
        }
    }

    Ok(())
}

/// Saves tasks to JSON file with pretty-printing
/// Why: Persistent storage between program runs
fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string_pretty(&tasks)?;  // Pretty-print for readability
    std::fs::write("tasks.json", serialized)?;  // Write to disk
    Ok(())
}

/// Loads tasks from JSON file, creates empty file if none exists
/// Why: Maintain task list between sessions
fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    // Try to read file, fallback to empty array if file doesn't exist
    let data = std::fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    let tasks: Vec<Task> = serde_json::from_str(&data)?;  // Deserialize JSON
    Ok(tasks)
}

/// Adds new task with auto-incremented ID
/// Why: Ensure unique identifiers for each task
fn add_task(tasks: &mut Vec<Task>, content: String) {
    let new_task = Task {
        id: tasks.len() as u32 + 1,  // Simple ID generation
        content,
        completed: false,
    };
    tasks.push(new_task);
}

/// Displays tasks in a user-friendly format
/// Why: Make task status easily readable
fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("📭 Your todo list is empty!");
        return;
    }

    println!("📋 Your Todo List:");
    for task in tasks {
        let status = if task.completed { "✓" } else { " " };  // Visual completion indicator
        println!("{:3} [{}] {}", task.id, status, task.content);
    }
}

/// Marks task as complete by ID
/// Why: Allow users to track progress
fn check_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        true  // Return success status
    } else {
        false  // Return failure if ID not found
    }
}

/// Removes task by ID
/// Why: Allow list maintenance
fn delete_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
        true  // Return success status
    } else {
        false  // Return failure if ID not found
    }
}