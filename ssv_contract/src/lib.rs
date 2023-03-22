extern crate core;

pub mod event;
pub mod contract;
pub mod decode;
pub mod ssv_contract;

use log::{error};

pub fn test_log_log() {
    error!("test_logtest_logtest_logtest_log")
}


#[cfg(test)]
mod test {

    use env_logger;
    use log::{debug, error, info, warn};
    use crate::test_log_log;

    fn init() {
        // env_logger::init()
        env_logger::builder().is_test(true).init();
        error!("inited error log");
        println!("inited")
    }

    #[test]
    fn test_log() {
        env_logger::builder()
            .is_test(true).init();
        error!("aaaaa bbb");
        test_log_log();
        println!("ended")
    }
}
