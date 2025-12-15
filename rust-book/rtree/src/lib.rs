use std::fs;
use std::path::Path;

pub struct Config {
    pub target_path: String,
}

impl Config {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the first argument (program name)

        // If the user provides a path, use it. Otherwise default to "."
        let target_path = match args.next() {
            Some(arg) => arg,
            None => String::from("."),
        };

        Ok(Config { target_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&config.target_path);
    // This line creates a new `Path` object from the `target_path` string in the `config`.
    // The `Path` type provides methods for interacting with file system paths.

    if path.is_dir() {
        // We need a recursive function separate from run() to handle directories
        visit_dirs(path, "")?;
    } else {
        // If it's a file, we can just print it
        println!("{}", path.display());
    }

    Ok(())
}

// The Core Recursive logic
fn visit_dirs_basic(dir: &Path, prefix: &str) -> std::io::Result<()> {
    // 1. Is this actually a directory? If not, we can't look inside it.
    if dir.is_dir() {
        // 2. Get a list of every item (file or folder) inside this directory.
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // 3. Extract the actual name (e.g., "main.rs") from the full path.
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    // 4. PRINT IT.
                    // We print the 'prefix' (the indentation) followed by the name.
                    println!("{}{}", prefix, filename_str);

                    // RIGHT NOW, this function stops here.
                    // It lists the files, but it doesn't dive deeper yet.
                    // In the next step, we will add code HERE to call visit_dirs AGAIN.
                }
            }
        }
    }
    Ok(())
}

pub fn visit_dirs(dir: &Path, prefix: &str) -> std::io::Result<()> {
    if dir.is_dir() {
        let entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        // We need to know if we are on the LAST item to choose └── vs ├──
        let count = entries.len();

        for (index, path) in entries.iter().enumerate() {
            // Check if this is the last item in the current folder
            let is_last = index == count - 1;

            // Choose the correct connector
            let connector = if is_last { "└── " } else { "├── " };

            // Get the file name
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    // Print the current item
                    println!("{}{}{}", prefix, connector, filename_str);

                    // RECURSION: If this item is a directory, dive into it!
                    if path.is_dir() {
                        // Prepare the prefix for the children
                        // If we are last, children don't need the vertical bar │
                        let new_prefix = if is_last {
                            format!("{}    ", prefix)
                        } else {
                            format!("{}│   ", prefix)
                        };

                        // Call the function again with the new path and new prefix
                        visit_dirs(&path, &new_prefix)?;
                    }
                }
            }
        }
    }
    Ok(())
}
