mod temperature;

use crate::temperature::*;
use std::env;

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
