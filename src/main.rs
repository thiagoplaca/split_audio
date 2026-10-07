use std::process::{exit};
use std::env;
use std::error::Error;

use split_audio::{download_audio_youtube, audio_separator, is_youtube};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    if let Err(e) = is_youtube(&config.url) {
        println!("Problem url: {e}");
        exit(1);
    };

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        exit(1);
    };

    Ok(())
}



fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let music_file = download_audio_youtube(&config.url)?;
    audio_separator(&music_file)?;

    println!("Processo concluído com sucesso: {}", music_file);

    Ok(())
    
}

struct Config {
    url: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let url = args[1].clone();

        Ok(Config { url })
    }
}


