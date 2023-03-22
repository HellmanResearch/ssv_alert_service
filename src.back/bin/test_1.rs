use std::ops::Deref;
use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};
use tracing::{error, info, warn};
use std::time::Duration;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct User {
    age: u16,
}

#[tokio::main]
async fn main() {
    let u = User{
        age: 23,
    };
    let a = Arc::new(u);
    let b = a.clone();
    let c = a.deref();
    println!("a.age: {}", a.age);
}