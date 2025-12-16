use std::fs;
use std::path::Path;

pub struct Config {
    pub target_path: String,
}

impl Config {
    pub fn build(args: &mut Vec<String>) -> Result<Config, &'static str> {
        // Skip the first argument (program name)
        let target_path = match args.get(1) {
            Some(arg) => arg.clone(),
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
        // FIX: We convert the DirEntry items into PathBuf items right here
        let entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let count = entries.len();

        for (index, path) in entries.iter().enumerate() {
            let is_last = index == count - 1;
            let connector = if is_last { "└── " } else { "├── " };

            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    println!("{}{}{}", prefix, connector, filename_str);

                    if path.is_dir() {
                        let new_prefix = if is_last {
                            format!("{}    ", prefix)
                        } else {
                            format!("{}│   ", prefix)
                        };

                        // Now this works because 'path' is truly a PathBuf
                        visit_dirs(&path, &new_prefix)?;
                    }
                }
            }
        }
    }
    Ok(())
}
