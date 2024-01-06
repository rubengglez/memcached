use crate::types::{READ_COMMANDS, WRITE_COMMANDS};

pub struct CommandParserInputData {
    pub command: String,
    pub key: String,
    pub value: Option<String>,
    pub flags: Option<u16>,
    pub value_size_bytes: Option<usize>,
    pub exptime: Option<isize>,
    pub no_reply: Option<bool>,
}

impl CommandParserInputData {
    pub fn from_string(data: String) -> Result<CommandParserInputData, String> {
        let mut iterator = data.split_whitespace();
        let size = iterator.clone().count();
        let command = iterator.next().unwrap();
        let key = iterator.next();
        if key.is_none() {
            return Err(String::from(format!(
                "Wrong number of arguments for {command}"
            )));
        }
				let key = key.unwrap();

        if WRITE_COMMANDS.iter().any(|&rc| rc == command) {
            if size != 5 && size != 6 {
                return Err(String::from(format!(
                    "Wrong number of arguments for {command}"
                )));
            }

            let flags: u16 = iterator.next().unwrap().parse().unwrap();
            let exptime: isize = iterator.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = iterator.next().unwrap().parse().unwrap();
            let no_reply = iterator.next();

            Ok(CommandParserInputData {
                command: command.to_owned(),
                key: key.to_owned(),
                value: None,
                flags: Some(flags),
                value_size_bytes: Some(value_size_in_bytes),
                exptime: Some(exptime),
                no_reply: Some(no_reply.is_some()),
            })
        } else if READ_COMMANDS.iter().any(|&rc| rc == command) {
            if size != 2 {
                return Err(String::from(format!(
                    "Wrong number of arguments for {command}"
                )));
            }

            Ok(CommandParserInputData {
                command: command.to_owned(),
                key: key.to_owned(),
                value: None,
                flags: None,
                value_size_bytes: None,
                exptime: None,
                no_reply: None,
            })
        } else {
            return Err(String::from("Wrong command"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_command() {
        let data = String::from("wrong command");
        let result = CommandParserInputData::from_string(data);
        assert!(result.is_err());
    }

    #[test]
    fn should_parse_get_command() {
        let data = String::from("get test\r\n");
        let result = CommandParserInputData::from_string(data);
        assert!(result.is_ok());
				let obj = result.unwrap();
        assert_eq!(obj.command, "get");
        assert_eq!(obj.key, "test");
    }

    #[test]
    fn should_raise_wrong_arguments_for_get_command_due_to_missing_arg() {
        let data = String::from("get\r\n");
        let result = CommandParserInputData::from_string(data);
        assert!(result.is_err());
    }

		 #[test]
    fn should_raise_wrong_arguments_for_get_command_due_to_more_args_than_expected() {
        let data = String::from("get test lala\r\n");
        let result = CommandParserInputData::from_string(data);
        assert!(result.is_err());
    }
}
