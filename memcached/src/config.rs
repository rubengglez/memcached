use crate::errors::*;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Server {
    default_port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Settings {
    server: Server,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    pub separator: String,
}

impl Protocol {
    fn create(s: &Config) -> Protocol {
        Protocol {
            separator: s.get("protocol.separator").unwrap(),
        }
    }
}

pub struct MyConfig {
    pub port: u16,
    pub protocol: Protocol,
}

pub struct Options {
    config_file: String,
}

impl Options {
    fn default() -> Options {
        Options {
            config_file: "memcached/config/default".to_string(),
        }
    }
}

impl MyConfig {
    pub fn parse(
        mut args: impl Iterator<Item = String> + ExactSizeIterator,
        opt: Option<Options>,
    ) -> Result<MyConfig, Errors> {
        let mut options = opt;
        if options.is_none() {
            options = Some(Options::default());
        }

        let s = Config::builder()
            .add_source(File::with_name(&options.unwrap().config_file))
            .build()?;

        let protocol = Protocol::create(&s);

        if args.len() != 1 && args.len() != 3 {
            return Err(Errors::InvalidNumberArguments(String::from(
                "Invalid number of arguments",
            )));
        }

        args.next();

        if args.len() == 0 {
            return Ok(MyConfig {
                port: s.get::<u16>("server.default_port").unwrap(),
                protocol,
            });
        }

        let option = args.next();

        if option.is_none() || !String::from("-p").eq(&option.unwrap()) {
            return Err(Errors::InvalidOptionalArguments(String::from(
                "Invalid optional argument",
            )));
        }

        let port: u16 = match args.next().unwrap().parse() {
            Err(_) => {
                return Err(Errors::InvalidGivenPort(String::from(
                    "Value given is not a valid port. Provide a integer",
                )))
            }
            Ok(p) => p,
        };

        Ok(MyConfig { port, protocol })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn should_fail_when_args_is_empty() -> Result<(), String> {
        let args = vec![].into_iter();

        match MyConfig::parse(args.into_iter(), None) {
            Ok(_) => Err(String::from("Empty args are not allowed")),
            Err(value) => match value {
                Errors::InvalidNumberArguments(_) => Ok(()),
                _ => Err(String::from("empty args are not allowed")),
            },
        }
    }

    #[test]
    fn should_use_default_when_optional_port_is_not_given() -> Result<(), Box<dyn Error>> {
        let args = ["myProgram"].iter().map(|s| s.to_string());
        let s = Config::builder()
            .add_source(File::with_name("config/test"))
            .build()?;
        let options = Some(Options {
            config_file: "config/test".to_string(),
        });

        match MyConfig::parse(args.into_iter(), options) {
            Ok(config) => {
                if config.port == s.get::<u16>("server.default_port").unwrap() {
                    Ok(())
                } else {
                    Err(String::from(format!(
                        "Expected port {}, got {}",
                        s.get::<u16>("server.default_port").unwrap(),
                        config.port
                    ))
                    .into())
                }
            }
            Err(_) => Err(String::from(format!(
                "Expected port {}",
                s.get::<u16>("server.default_port").unwrap()
            ))
            .into()),
        }
    }

    #[test]
    fn should_use_always_default_separator_protocol() -> Result<(), Box<dyn Error>> {
        let args = ["myProgram"].iter().map(|s| s.to_string());
        let s = Config::builder()
            .add_source(File::with_name("config/test"))
            .build()?;
        let options = Some(Options {
            config_file: "config/test".to_string(),
        });

        match MyConfig::parse(args.into_iter(), options) {
            Ok(config) => {
                if config.protocol.separator == s.get::<String>("protocol.separator").unwrap() {
                    Ok(())
                } else {
                    Err(String::from(format!(
                        "Expected separator {}, got {}",
                        s.get::<String>("protocol.separator").unwrap(),
                        config.protocol.separator,
                    ))
                    .into())
                }
            }
            Err(_) => Err(String::from(format!(
                "Expected port {}",
                s.get::<u16>("server.default_port").unwrap()
            ))
            .into()),
        }
    }

    #[test]
    fn should_fail_when_optional_param_is_given_but_not_value() -> Result<(), String> {
        let args = ["myProgram", "-p"].iter().map(|s| s.to_string());

        match MyConfig::parse(args.into_iter(), None) {
            Ok(_) => Err(String::from("Empty args are not allowed")),
            Err(value) => match value {
                Errors::InvalidNumberArguments(_) => Ok(()),
                _ => Err(String::from("Invalid number of arguments")),
            },
        }
    }

    #[test]
    fn should_fail_when_optional_param_is_different_than_expected() -> Result<(), String> {
        let args = ["myProgram", "-lolo", "1234"].iter().map(|s| s.to_string());

        match MyConfig::parse(args.into_iter(), None) {
            Ok(_) => Err(String::from("Empty args are not allowed")),
            Err(value) => match value {
                Errors::InvalidOptionalArguments(_) => Ok(()),
                _ => Err(String::from("Config should not be created")),
            },
        }
    }

    #[test]
    fn should_fail_when_optional_param_is_given_but_its_value_is_not_an_integer(
    ) -> Result<(), String> {
        let args = ["myProgram", "-p", "abcd"].iter().map(|s| s.to_string());

        match MyConfig::parse(args.into_iter(), None) {
            Ok(_) => Err(String::from("Empty args are not allowed")),
            Err(value) => match value {
                Errors::InvalidGivenPort(_) => Ok(()),
                _ => Err(String::from("Should provide a valid port")),
            },
        }
    }
}
