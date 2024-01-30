use crate::{
    config::Protocol,
    types::{READ_COMMANDS, WRITE_COMMANDS},
};

pub struct CommandParserInputDataBuilder {
    protocol: Protocol,
}

pub struct CommandParserInputData {
    pub command: String,
    pub key: String,
    pub value: Option<String>,
    pub flags: Option<u16>,
    pub value_size_bytes: Option<usize>,
    pub exptime: Option<isize>,
    pub no_reply: Option<bool>,
}

impl CommandParserInputDataBuilder {
    pub fn new(protocol: Protocol) -> CommandParserInputDataBuilder {
        CommandParserInputDataBuilder { protocol }
    }

    pub fn build(&self, data: String) -> Result<CommandParserInputData, String> {
        let command_and_data_list: Vec<&str> =
            data.split_terminator(&self.protocol.separator).collect();
        if command_and_data_list.len() != 1 && command_and_data_list.len() != 2 {
            tracing::info!(
                "command_and_data_list is {}, {:?}",
                command_and_data_list.len(),
                command_and_data_list
            );
            return Err(format!("Wrong number of arguments for {data}"));
        }
        let mut command_data = command_and_data_list[0].split_whitespace();
        let size = command_data.clone().count();
        let command = command_data.next().unwrap();
        let key = command_data.next();
        if key.is_none() {
            tracing::info!("key is none");
            return Err(format!("Wrong number of arguments for {command}"));
        }
        let key = key.unwrap();

        if WRITE_COMMANDS.iter().any(|&rc| rc == command) {
            if command_and_data_list.len() != 2 {
                tracing::info!("command_and_data_list is {:?}", command_and_data_list);
                return Err(format!("Wrong number of arguments for {command}"));
            }

            if size != 5 && size != 6 {
                tracing::info!("size is {}", size);
                return Err(format!("Wrong number of arguments for {command}"));
            }

            let flags: u16 = command_data.next().unwrap().parse().unwrap();
            let exptime: isize = command_data.next().unwrap().parse().unwrap();
            let value_size_in_bytes: usize = command_data.next().unwrap().parse().unwrap();
            let no_reply = command_data.next();
            let value = command_and_data_list[1];

            if value.len() != value_size_in_bytes {
                tracing::info!("value not matched expected");
                return Err("Value in bytes does not match expected".to_string());
            }

            Ok(CommandParserInputData {
                command: command.to_owned(),
                key: key.to_owned(),
                value: Some(value.to_owned()),
                flags: Some(flags),
                value_size_bytes: Some(value_size_in_bytes),
                exptime: Some(exptime),
                no_reply: Some(no_reply.is_some()),
            })
        } else if READ_COMMANDS.iter().any(|&rc| rc == command) {
            if command_and_data_list.len() != 1 {
                return Err(format!("Wrong number of arguments for {command}"));
            }
            if size != 2 {
                return Err(format!("Wrong number of arguments for {command}"));
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
            tracing::info!("Wrong command when parsing command");
            return Err(String::from("Wrong command"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_builder() -> CommandParserInputDataBuilder {
        CommandParserInputDataBuilder::new(Protocol {
            separator: String::from("--"),
        })
    }

    #[test]
    fn wrong_command() {
        let data = String::from("wrong command");
        let result = create_builder().build(data);
        assert!(result.is_err());
    }

    #[test]
    fn wrong_command_when_sending_empty_data() {
        let data = String::from("");
        let result = create_builder().build(data);
        assert!(result.is_err());
    }

    #[test]
    fn should_parse_get_command() {
        let data = String::from("get test--");
        let result = create_builder().build(data);
        assert!(result.is_ok());
        let obj = result.unwrap();
        assert_eq!(obj.command, "get");
        assert_eq!(obj.key, "test");
    }

    #[test]
    fn should_raise_wrong_arguments_for_get_command_due_to_missing_arg() {
        let data = String::from("get--");
        let result = create_builder().build(data);
        assert!(result.is_err());
    }

    #[test]
    fn should_raise_wrong_arguments_for_get_command_due_to_more_args_than_expected() {
        let data = String::from("get test lala--");
        let result = create_builder().build(data);
        assert!(result.is_err());
    }

    #[test]
    fn should_parse_set_command_with_reply() {
        let data = String::from("set test 0 100 4--hola--");
        let result = create_builder().build(data);
        assert!(result.is_ok());
        let obj = result.unwrap();
        assert_eq!(obj.command, "set");
        assert_eq!(obj.key, "test");
        assert_eq!(obj.exptime, Some(100));
        assert_eq!(obj.flags, Some(0));
        assert_eq!(obj.no_reply, Some(false));
        assert_eq!(obj.value, Some("hola".to_owned()));
        assert_eq!(obj.value_size_bytes, Some(4));
    }

    #[test]
    fn should_parse_set_command_with_no_reply() {
        let data = String::from("set test 0 100 4 no_reply--hola--");
        let result = create_builder().build(data);
        assert!(result.is_ok());
        let obj = result.unwrap();
        assert_eq!(obj.command, "set");
        assert_eq!(obj.key, "test");
        assert_eq!(obj.exptime, Some(100));
        assert_eq!(obj.flags, Some(0));
        assert_eq!(obj.no_reply, Some(true));
        assert_eq!(obj.value, Some("hola".to_owned()));
        assert_eq!(obj.value_size_bytes, Some(4));
    }

    #[test]
    fn should_raise_error_when_set_command_missing_argument() {
        let data = String::from("set test 0 100 --hola--");
        let result = create_builder().build(data);
        assert!(result.is_err());
    }

    #[test]
    fn should_raise_error_when_data_passed_to_set_command_is_different_size_than_expected() {
        let data = String::from("set test 0 100 4--hello--");
        let result = create_builder().build(data);
        assert!(result.is_err());
    }
}
