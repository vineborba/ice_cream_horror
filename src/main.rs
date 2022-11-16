use std::env;
use std::process;

use ice_cream_horror::Config;

fn main() {
    let config = Config::new(env::args());

    println!("Lendo arquivo {}", config.input_file);

    if let Err(e) = ice_cream_horror::run(&config) {
        eprintln!("Erro da aplicação: {}", e);
        process::exit(1);
    }
}
