mod sync_decided_2;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};
use tracing::{error, info, warn};
use std::time::Duration;
use std::sync::Arc;

use ethers_core::{
    abi::Abi,
    types::{Address, H256, U256},
};
#[derive(Clone, Copy)]
struct User {
    age: u16,
}

#[tokio::main]
async fn main() {
    let a = U256([3847028935231345152, 3, 0, 0]);
    let b = U256([1e16 as u64, 0, 0, 0, ]);

    let c = a / b;
    println!("{}", c);
    let r = log::LevelFilter::Info.to_string();
    println!("r: {r}");
    let file_path = "decided.txt";
    let mut decided_file = OpenOptions::new().write(true).append(true).create_new(!Path::new(file_path).exists())
        .open("decided.txt").unwrap();
    // if Path::new(file_path).exists() {
    //
    //     decided_file = OpenOptions::new().write(true).append(true)
    //         .open("decided.txt").unwrap();
    //     decided_file = File::open("decided.txt").unwrap();
    // }else {
    // }

    // let mut decided_file = File::open("decided.txt").unwrap();
    decided_file.write("abc\n".as_bytes()).unwrap();

}