#![deny(warnings)]

#[cfg(feature = "hosted")]
extern crate actix_web;
#[cfg(feature = "hosted")]
#[macro_use]
extern crate lazy_static;

extern crate fern;
#[macro_use]
extern crate log;
extern crate clap;
#[cfg(feature = "hosted")]
#[macro_use]
extern crate structopt;
extern crate chrono;
extern crate serde;
#[cfg(feature = "hosted")]
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate futures;
extern crate select;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate uuid;

extern crate episod;

#[cfg(feature = "hosted")]
mod hosted_helpers;

#[cfg(feature = "hosted")]
lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: hosted_helpers::config::Config = hosted_helpers::config::Config::load();
}

#[cfg(feature = "hosted")]
pub fn start_server() {
    info!("Starting episod with config: {:?}", *CONFIG);

    match std::env::var("LISTEN_FD") {
        Ok(fd) => hosted_helpers::api::serve_from_fd(&fd),
        _ => hosted_helpers::api::serve(&CONFIG.host, CONFIG.port),
    }
}

fn main() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("episod", log::LevelFilter::Trace)
        .level_for("tokio_core", log::LevelFilter::Error)
        .level_for("mio", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.9f"),
                record.target(),
                record.level(),
                message
            ))
        }).apply()
        .unwrap();

    #[cfg(feature = "hosted")]
    start_server();

    info!("Goodbye");
}
