pub struct Config {
    pub input_base: Base,
    pub input: String,
    pub output_base: Base,
}

pub enum Base {
    Binary,
    Decimal,
    Hexadecimal,
}

impl Base {
    fn parse(s: &str) -> Result<Base, &'static str> {
        match s {
            "2" | "bin" => Ok(Base::Binary),
            "10" | "dec" => Ok(Base::Decimal),
            "16" | "hex" => Ok(Base::Hexadecimal),
            _ => Err("unsupported base"),
        }
    }

    fn radix(&self) -> u32 {
        match self {
            Base::Binary => 2,
            Base::Decimal => 10,
            Base::Hexadecimal => 16,
        }
    }
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input_base: Base = match args.next() {
            Some(arg) => Base::parse(&arg)?,
            None => return Err("didn't get an input base"),
        };

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get an input"),
        };

        let output_base: Base = match args.next() {
            Some(arg) => Base::parse(&arg)?,
            None => return Err("didn't get an output base"),
        };

        Ok(Config {
            input_base,
            input,
            output_base,
        })
    }
}

fn convert(config: &Config) -> Result<String, &'static str> {
    let number = u32::from_str_radix(&config.input, config.input_base.radix())
        .map_err(|_| "invalid input for base")?;

    let result = match config.output_base {
        Base::Binary => format!("{:b}", number),
        Base::Decimal => format!("{}", number),
        Base::Hexadecimal => format!("{:X}", number),
    };

    Ok(result)
}

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
