use log::LevelFilter;
use log::{error, info};
use simple_logger::SimpleLogger;

use app::App;

mod app;

fn main() {
    SimpleLogger::new()
        .with_module_level("rustls", LevelFilter::Info)
        .with_module_level("ureq", LevelFilter::Info)
        .init()
        .unwrap();

    info!("statusng version {}.", statusng::VERSION);

    let app = match App::build() {
        Ok(app) => app,
        Err(err) => {
            error!("Failed to load app: {:?}", err);
            std::process::exit(-1);
        }
    };

    info!("Loaded app.");
    app.run();
}
