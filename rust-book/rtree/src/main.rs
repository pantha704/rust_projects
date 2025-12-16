// use num_cpus;
use rtree::run;
use rtree::Config;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

#[allow(dead_code)]
// This attribute globally disables the `dead_code` lint for the entire crate.
// The `dead_code` lint warns about unused functions, methods, structs, enums,
// and other items. Disabling it can be useful during development when
// code might temporarily be unused, or in libraries where not all public
// items are expected to be used by every consumer.

const TOTAL_CPU_THREADS: usize = 4;

struct DirStats {
    file_count: usize,
    total_size: u64,
}

fn main() {
    let mut args = env::args().collect::<Vec<String>>();

    let config = Config::build(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Target directory: {}\n", config.target_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // PARALLEL SCAN
    // 1. Setup Shared State
    let stats = Arc::new(Mutex::new(DirStats {
        file_count: 0,
        total_size: 0,
    }));

    // 2. Setup ThreadPool (Limit to 4 or 8 threads to save your OS resources or just use num_cpus::get() as usize)
    let pool = ThreadPool::new(TOTAL_CPU_THREADS);

    // 3. Start parallel scan
    // We need to pass a CLONE of the Arc, and the pool itself
    let target_str = if args.len() > 1 { &args[1] } else { "." };
    let target_dir = PathBuf::from(target_str);
    scan_dir(target_dir.clone(), stats.clone(), pool.clone());

    // 4. WAIT. If we don't wait, main exits and kills the threads instantly
    pool.join();

    // 5. Print results
    let final_sets = stats.lock().unwrap();
    println!(
        "Scanned {} files with total size: {} bytes or {}",
        final_sets.file_count,
        final_sets.total_size,
        format_size(final_sets.total_size)
    );

    fn scan_dir(dir: PathBuf, stats: Arc<Mutex<DirStats>>, pool: ThreadPool) {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => {
                // eprintln!("Failed to read directory: {e}");
                return;
            }
        };

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // RECURSION IN PARALLEL
                    // We found a folder. Instead of driving in immediately,
                    // we throw this job into the ThreadPool.

                    // 1. Clone the Arc so the new thread gets its own pointer
                    let stats_clone = Arc::clone(&stats);

                    // 2. Clone the pool so the new thread can spawn more threads
                    let pool_clone = pool.clone();

                    // 3. Spawn a new thread
                    pool.execute(move || {
                        scan_dir(path, stats_clone, pool_clone);
                    });
                } else {
                    // FOUND A FILE
                    if let Ok(metadata) = entry.metadata() {
                        let size = metadata.len();

                        // lock the stats and update
                        let mut stats = stats.lock().unwrap();
                        stats.file_count += 1;
                        stats.total_size += size;
                        // exiting the thread
                    }
                }
            }
        }
    }
}

fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let size = bytes as f64;

    if size < KB {
        format!("{} B", bytes)
    } else if size < MB {
        format!("{:.2} KB", size / KB)
    } else if size < GB {
        format!("{:.2} MB", size / MB)
    } else {
        format!("{:.2} GB", size / GB)
    }
}
