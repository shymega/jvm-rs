extern crate log;
extern crate log4rs;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config as Log4rsConfig, Root};

fn init_logger_default() {
    let console_appender = Appender::builder().build("console".to_owned(),
                                                     Box::new(ConsoleAppender::builder().build()));
    let root = Root::builder()
        .appender("console".to_owned())
        .build(LogLevelFilter::Info);

    let log4rs_config = Log4rsConfig::builder()
        .appender(console_appender)
        .build(root)
        .expect("Unable to build log4rs configuration.");

    log4rs::init_config(log4rs_config).expect("log4rs load failed..");

    warn!("Could not load logging configuration.");
    warn!("Please note I used my defaults. Please fix this error.");
}

pub fn init_logger(path: &str) {
    match log4rs::init_file(path.clone(), Default::default()) {
        Ok(_) => debug!("Loaded log4rs.."),
        Err(_) => {
            init_logger_default();
        }
    };
}
