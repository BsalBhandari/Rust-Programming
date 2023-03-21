use std::env;
use std::process;
use CLI::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err | {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = CLI::run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
    
}


