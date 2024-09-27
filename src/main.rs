use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::spawn;
use std::time::Duration;

fn main() {
    let completed_jobs = Arc::new(Mutex::new(0));
    let num_workers = 10;
    let mut handles = vec![];
    let jobs_per_worker = 3;
    for worker_id in 0..num_workers{
        let completed_jobs = Arc::clone(&completed_jobs);
        let handle = spawn(move || {
            for job_id in 0..jobs_per_worker{
                println!("Worker {} works on {}", worker_id, job_id);
                thread::sleep(Duration::from_millis(500));
                let mut completed_works = completed_jobs.lock().unwrap();
                *completed_works += 1;
                println!("Worker {} finished {}", worker_id, job_id);
            }
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Work finished")
}
