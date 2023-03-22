mod test_contract;

use tokio;
use env_logger;
use log::{info, warn, error, debug};

fn test_log_print() {
    info!("aaaaaa")
}

fn main() {
    let t = 234;
    println!("aaa");
}


#[cfg(test)]
mod test {
    use log::{error, info};
    use crate::test_log_print;

    fn init() {
        // env_logger::init()
        env_logger::builder().is_test(true).init();
    }

    #[tokio::test]
    async fn test_log() {
        init();
        error!("starting...");
        println!("p starting...");
        test_log_print()
    }
}