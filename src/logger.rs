use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub struct Logger;

impl Logger {
    pub fn init(verbose: bool) {
        let log_level = if verbose {
            Level::DEBUG
        } else {
            Level::INFO
        };

        let subscriber = FmtSubscriber::builder()
            .with_max_level(log_level)
            .with_target(false)
            .without_time()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Setting default subscriber failed");
    }
}