

use std::time::Duration;

pub enum Status {
    INIT,
    RUNNING,
    STOP
}

pub struct Job<T> {
    status: Status,
    task_function: T,
    intervals: Duration
}

impl<T> Job<T> {
    pub fn new(task_function: T, intervals: Duration) -> Self {
        return Self {
            status: Status::INIT,
            task_function,
            intervals,
        }
    }

    // pub fn start(&self) -> Result<(), String>{
    //     return match self.status {
    //         Status::INIT => {
    //             tokio::spawn(async {
    //                 loop {
    //                     tokio::spawn(self.task_function);
    //                     tokio::time::sleep(self.intervals)
    //                 }
    //             });
    //             Ok(())
    //         }
    //         _ => {
    //             Err("Unknown status".to_string())
    //         }
    //     }
    // }
}