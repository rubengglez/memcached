// TODO: Use ENV VAR to get this value
const DEFAULT_PORT: u16 = 11211;

pub struct Config {
    pub port: u16,
}

// TODO: create different errors instead of returning Err(String)
impl Config {
    pub fn parse(
        mut args: impl Iterator<Item = String> + ExactSizeIterator,
    ) -> Result<Config, String> {
        if args.len() != 1 && args.len() != 3 {
            return Err(String::from("Invalid number of arguments"));
        }

        args.next();

        if args.len() == 0 {
            return Ok(Config { port: DEFAULT_PORT });
        }

        let option = args.next();

        if option.is_none() || !String::from("-p").eq(&option.unwrap()) {
            return Err(String::from("Invalid optional argument"));
        }

        let port: u16 = match args.next().unwrap().parse() {
            Err(_) => {
                return Err(String::from(
                    "Value given is not a valid port. Provide a integer",
                ))
            }
            Ok(p) => p,
        };

        Ok(Config { port })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn should_fail(
        args: impl Iterator<Item = String> + ExactSizeIterator,
        message_when_not_fails: &str,
        error_expected: &str,
    ) -> Result<(), String> {
        match Config::parse(args.into_iter()) {
            Ok(_) => Err(String::from(message_when_not_fails)),
            Err(value) => {
                if value == error_expected {
                    Ok(())
                } else {
                    Err(String::from(format!(
                        "Expected returned error is: {} and got: {}",
                        error_expected, value
                    )))
                }
            }
        }
    }

    #[test]
    fn should_fail_when_args_is_empty() -> Result<(), String> {
        let args = vec![].into_iter();

        should_fail(
            args,
            "Empty args are not allowed",
            "Invalid number of arguments",
        )
    }

    #[test]
    fn should_use_default_when_optional_port_is_not_given() -> Result<(), String> {
        let args = ["myProgram"].iter().map(|s| s.to_string());

        match Config::parse(args.into_iter()) {
            Ok(config) => {
                if config.port == DEFAULT_PORT {
                    Ok(())
                } else {
                    Err(String::from(format!("Expected port {}", DEFAULT_PORT)))
                }
            }
            Err(_) => Err(String::from(format!("Expected port {}", DEFAULT_PORT))),
        }
    }

    #[test]
    fn should_fail_when_optional_param_is_given_but_not_value() -> Result<(), String> {
        let args = ["myProgram", "-p"].iter().map(|s| s.to_string());

        should_fail(
            args,
            "Config should not be created",
            "Invalid number of arguments",
        )
    }

    #[test]
    fn should_fail_when_optional_param_is_different_than_expected() -> Result<(), String> {
        let args = ["myProgram", "-lolo", "1234"].iter().map(|s| s.to_string());

        should_fail(
            args,
            "Config should not be created",
            "Invalid optional argument",
        )
    }

    #[test]
    fn should_fail_when_optional_param_is_given_but_its_value_is_not_an_integer(
    ) -> Result<(), String> {
        let args = ["myProgram", "-p", "abcd"].iter().map(|s| s.to_string());

        should_fail(
            args,
            "Config should not be created",
            "Value given is not a valid port. Provide a integer",
        )
    }
}
