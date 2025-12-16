# rtree

rtree is a high-performance directory scanner and visualizer written in Rust. It parallelizes the scanning process inside every folder to efficiently calculate the size of all files.

## Description

rtree combines directory visualization with multi-threaded size calculation. It uses a **thread pool** for thread parallelization and management, effectively distributing the workload across CPU threads (currently configured to use 4 threads) to speed up deep directory traversals.

The tool provides:

1. A visual tree structure of the directory (similar to the standard `tree` command).
2. A summary with the total number of files found and their cumulative size.

## Features

- **Parallel Scanning**: Uses a thread pool to concurrently scan subdirectories.
- **Disk Usage Calculation**: Aggregates the size of all files recursively.
- **Tree Visualization**: Prints a formatted directory tree to the console.
- **Thread Safety**: Implements `Arc` and `Mutex` for safe shared state management across threads.

## Usage

To run the program, use `cargo run` and optionally provide a target directory path.

### basic usage

Scans the current directory (`.`):

```bash
cargo run
```

### Scan a specific directory

Pass the path as an argument:

```bash
cargo run -- /path/to/directory
```

## Example Output

```text
Target directory: ./src

├── main.rs
└── lib.rs

Scanned 2 files with total size: 7201 bytes or 7.03 KB
```

## Configuration

The concurrency limit is currently set to **4 threads** via the `TOTAL_CPU_THREADS` constant in `src/main.rs`. You can modify this value in the source code to adjust based on your machine's capabilities.

## Dependencies

- [threadpool](https://crates.io/crates/threadpool): For managing the pool of worker threads.
