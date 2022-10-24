use clap::Parser;
use crate::config::{GLOBAL_CONFIG, Config};
use std::env;
use crate::backend::Backend;
use crate::backend::{baidu_trans, youdao_trans};

#[derive(Debug, Default)]
pub struct App {
    config: Config,
    backend: Option<Box<dyn Backend>>,
}

impl App {
    pub fn run() {
        /* create an instance */
        let mut app = App::new();

        /* initialize global environment */
        app.init();

        /* initialize cli parser */
        let cli = Cli::parse();

        if let Some(path) = cli.config {
            unsafe { GLOBAL_CONFIG = path }
        }

        if let (Some(from), Some(to)) = (&cli.from, &cli.to) {
            app.config.basic.from = from.to_owned();
            app.config.basic.to = to.to_owned();
        } if let (None, Some(_)) = (&cli.from, &cli.to) {
            println!("Number of arguments isn't enough.")
        } if let (Some(_), None) = (&cli.from, &cli.to) {
            println!("Number of arguments isn't enough.")
        }

        if let Some(text) = cli.text {
            match app.config.basic.backend.as_str() {
                "default" => {
                    app.backend = Some(Box::new(baidu_trans::Backend::new(&app.config.key.appid, &app.config.key.secret_key)));
                },
                "youdao" => {
                    app.backend = Some(Box::new(youdao_trans::Backend::new(&app.config.key.appid, &app.config.key.secret_key)));
                }
                _ => {}
            }
            app.trans(text);
        }

    }
    
    fn new() -> Self {
        Self::default()
    }

    fn init(&mut self) {
        /* initialize configuration */
        let mut conf_path = "";
        unsafe {
            GLOBAL_CONFIG = ".config/trans-cli-rs/config.toml".to_owned();
            if let Ok(val) = env::var("HOME") {
                GLOBAL_CONFIG = val + "/" + GLOBAL_CONFIG.as_ref() ;
            }
            conf_path = GLOBAL_CONFIG.as_str();
        }
        self.config = Config::load(conf_path.to_owned()).expect("Please specify \"config.toml\" first.");
    }

    fn trans(&mut self, text: String) {
        /* normalize text */
        let mut normalized_text: Vec<u8> = vec![];
        let vec_text = text.as_bytes();
        for i in 0..vec_text.len() {
            match vec_text[i] as char {
                '\n' => { normalized_text.push(' ' as u8) },
                '\r' => {},
                '\t' => { normalized_text.push(' ' as u8) },
                _ => { normalized_text.push(vec_text[i])}
            }
        }
        let text = String::from_utf8(normalized_text).expect("Normalizing failed!");
        let data = self.backend.as_mut().unwrap().send_req(&self.config.basic.from, &self.config.basic.to, text).expect("Translating failed.");
        let dest = self.backend.as_ref().unwrap().handle_response(data);
        println!("\x1b[0;32m{}\x1b[0m", dest);
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

    /// specify a kind of source languages
    #[arg(short, long)]
    from: Option<String>,

    /// specify a kind of direction languages
    #[arg(short, long)]
    to: Option<String>,

    /// text that need to be translated
    text: Option<String>,
}
