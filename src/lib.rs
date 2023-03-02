use std::env;

#[derive(Debug, PartialEq)]
enum Temperature {
    C(f32),
    F(f32),
}

impl Temperature {
    pub fn convert(&self) -> Temperature {
        match self {
            Temperature::C(val) => {
                let result = val * (9.0 / 5.0) + 32.0;
                Temperature::F((result * 100.0).round() / 100.0)
            }
            Temperature::F(val) => {
                let result = (val - 32.0) * 5.0 / 9.0;
                Temperature::C((result * 100.0).round() / 100.0)
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Temperature::C(val) => format!("{} C", val),
            Temperature::F(val) => format!("{} F", val),
        }
    }
}

pub struct Config {
    to: Option<Temperature>,
    value: f32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let value = match args.next() {
            Some(v) => match v.parse::<f32>() {
                Ok(v) => v,
                Err(_) => return Err("Invalid value"),
            },
            None => return Err("No input value"),
        };

        let to = match args.next() {
            Some(temp) => {
                if temp == "c" || temp == "C" {
                    Some(Temperature::C(value))
                } else if temp == "f" || temp == "F" {
                    Some(Temperature::F(value))
                } else {
                    None
                }
            }
            None => None,
        };

        Ok(Self { to, value })
    }
}

pub fn run(config: Config) {
    match config.to {
        Some(temp) => {
            println!("{} -> {}", temp.to_string(), temp.convert().to_string());
        }
        None => {
            let c_temp = Temperature::C(config.value);
            let f_temp = Temperature::F(config.value);

            println!("{} -> {}", c_temp.to_string(), c_temp.convert().to_string());
            println!("{} -> {}", f_temp.to_string(), f_temp.convert().to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_convertion_works_correctly() {
        assert_eq!(Temperature::F(3.0).convert(), Temperature::C(-16.11));
        assert_eq!(Temperature::F(30.0).convert(), Temperature::C(-1.11));

        assert_eq!(Temperature::C(3.0).convert(), Temperature::F(37.4));
        assert_eq!(Temperature::C(30.0).convert(), Temperature::F(86.0));
    }
}
