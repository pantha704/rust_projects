// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    use std::sync::Mutex;
    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared: Arc<Mutex<JobStatus>> = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    
    // .lock() is needed because Mutex requires explicit locking to access its contents
    // .unwrap() handles the case where the lock might be poisoned (if another thread panicked while holding the lock)
    // This ensures thread-safe access to the shared jobs_done value
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
