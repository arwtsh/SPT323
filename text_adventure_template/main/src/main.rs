use assets;

use log::info;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn main() {
    //Set up logging in the app
    setup_logging();

    info!("Starting application...");

    assets::init_app();
}

fn setup_logging() {
    const OUTPUT_PATH: &str = "log/output.log";
    const OLD_OUTPUT_PATH: &str = "log/old_output.log";

    //Delete the second to last session's output log.
    let _deletion = std::fs::remove_file(OLD_OUTPUT_PATH);

    //Rename last session's output if it still exists.
    let _rename = std::fs::rename(OUTPUT_PATH, OLD_OUTPUT_PATH);
    
    //Initialize log4rs
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} {M} - {m}{n}")))
        .build(OUTPUT_PATH)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Trace))
    .unwrap();
    log4rs::init_config(config).expect("Failed to set up logger.");
}