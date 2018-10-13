use structopt::StructOpt;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            host: "0.0.0.0".to_string(),
            port: 7878,
        }
    }
}

impl Config {
    pub fn load() -> Config {
        let config = merge_configs();
        match config {
            Ok(config) => config,
            Err(err) => panic!("{:?}", err),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConfigLoader {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize, StructOpt)]
pub struct ConfigLoaderCmd {
    #[structopt(
        short = "h",
        long = "host",
        env = "HOST",
        help = "Listen on the specified host, by default 0.0.0.0"
    )]
    pub host: Option<String>,
    #[structopt(
        short = "p",
        long = "port",
        env = "PORT",
        help = "Listen on the specified host, by default 7878"
    )]
    pub port: Option<u16>,
}

fn merge_configs() -> Result<Config, String> {
    let from_args = ConfigLoaderCmd::from_args();
    let default = Config::default();

    Ok(Config {
        port: from_args.port.unwrap_or(default.port),
        host: from_args.host.unwrap_or(default.host),
    })
}
