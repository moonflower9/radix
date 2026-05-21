use radix::{Config, convert};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });

    match convert(&config) {
        Ok(result) => println!("{}", result),
        Err(err) => {
            eprintln!("conversion error: {}", err);
            std::process::exit(1);
        }
    }
}
