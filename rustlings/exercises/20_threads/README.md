# Threads

In most current operating systems, an executed program's code is run in a process, and the operating system manages multiple processes at once.
Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

## Further information

- [Dining Philosophers example](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html)
- [Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

Here's a beginner-friendly summary of key threading concepts in Rust:

### 1. Basic Thread Creation

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from new thread!");
    });

    println!("Hello from main thread!");
    handle.join().unwrap(); // Wait for thread to finish
}
```

**Key Points:**

- Use `thread::spawn` to create new threads
- Main thread exits when main function ends (unless you `join`)
- Always `join` threads to prevent premature exiting

---

### 2. Moving Ownership to Threads

```rust
let value = String::from("hello");
let handle = thread::spawn(move || {
    println!("{}", value); // Now owns value
});
```

**Why `move`?**

- Transfers ownership to thread
- Required to use variables from outer scope
- Prevents dangling references

---

### 3. Message Passing (Channels)

```rust
use std::sync::mpsc; // Multiple Producer, Single Consumer

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("Message")).unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());
}
```

**Channel Types:**

- `mpsc::channel()`: Creates a channel pair (transmitter, receiver)
- `send()`: Send message (blocks if buffer full)
- `recv()`: Receive message (blocks until message arrives)

---

### 4. Shared State Concurrency

```rust
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

**Key Components:**

- `Arc`: Atomic Reference Counter (thread-safe Rc)
- `Mutex`: Mutual exclusion (lock before accessing data)
- Always lock mutex before accessing data

---

### 5. Thread Safety Principles

1. **Send Trait:** Types that can be transferred between threads
2. **Sync Trait:** Types that can be referenced across threads
3. **Basic Rules:**
   - Immutable references are safe to share (`&T` is Sync)
   - Mutable references require synchronization (`&mut T` is Send but not Sync)

---

### Common Thread Patterns

| Pattern         | Use Case               | Example                     |
| --------------- | ---------------------- | --------------------------- | --- | ------- |
| Fire-and-forget | Background tasks       | `thread::spawn(             |     | {...})` |
| Worker threads  | Parallel processing    | Thread pool pattern         |
| Pipeline        | Data processing stages | Multiple connected channels |
| Shared state    | Concurrent data access | `Arc<Mutex<T>>`             |

---

### Important Modules

```rust
std::thread   // Thread management
std::sync::mpsc  // Message channels
std::sync::{Arc, Mutex, RwLock}  // Shared state
std::sync::atomic  // Atomic operations
```

---

### Thread Safety Cheat Sheet

| Type       | Send? | Sync? | Common Use                 |
| ---------- | ----- | ----- | -------------------------- |
| `i32`      | Yes   | Yes   | Simple values              |
| `String`   | Yes   | Yes   | Text data                  |
| `Rc<T>`    | No    | No    | Single-thread ref count    |
| `Arc<T>`   | Yes   | Yes   | Thread-safe ref count      |
| `Mutex<T>` | Yes   | Yes   | Protected mutable access   |
| `Cell<T>`  | No    | No    | Single-thread interior mut |

---

### Beginner Tips

1. Start with channels before shared state
2. Use `Arc` + `Mutex` for shared mutable state
3. Prefer message passing over shared state
4. Use `#[derive(Clone)]` for custom types to use in threads
5. Handle thread panics with `catch_unwind`

```rust
thread::spawn(|| {
    let result = panic::catch_unwind(|| {
        // Potentially dangerous code
    });
});
```

This covers the fundamentals you need to start working with threads in Rust. Remember: Rust's ownership system helps prevent data races, but you still need to choose the right synchronization tools!
