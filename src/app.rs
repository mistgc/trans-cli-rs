use clap::Parser;
use crate::config::GLOBAL_CONFIG;
use std::env;

pub struct App {}

impl App {
    pub fn run() {
        // initialize global environment
        App::init();

        // initialize cli parser
        let cli = Cli::parse();
        if let Some(path) = cli.config {
            unsafe { GLOBAL_CONFIG = path }
        }
        println!("App starting...");
        unsafe { println!("Config Path {}", GLOBAL_CONFIG) }
    }
    
    fn init() {
        unsafe {
            GLOBAL_CONFIG = ".config/trans-cli-rs/config.toml".to_owned();
            if let Ok(val) = env::var("HOME") {
                GLOBAL_CONFIG = val + "/" + GLOBAL_CONFIG.as_ref() ;
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    /// print all supported languages
    #[arg(short, long)]
    list: bool,

    /// specify config file
    #[arg(short, long)]
    config: Option<String>,

    /// text that need to be translated
    text: Option<String>,
}
