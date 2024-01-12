use std::{
    error::{self, Error},
    fmt,
};

type ResponseResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct InvalidResponseError;

impl fmt::Display for InvalidResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Response is invalid")
    }
}

impl error::Error for InvalidResponseError {}

pub fn parse_response(response: String) -> ResponseResult<String> {
    let list: Vec<&str> = response.trim().split("\r\n").collect();
    if list.len() < 2 {
        return Err(InvalidResponseError.into());
    }

    Ok(String::from(list[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_second_part_after_first_separator() {
        let data = "VALUE test 0 4\r\nhola\r\nEND\r\n".to_string();

        let result = parse_response(data);
        assert_eq!(result.unwrap(), "hola");
    }

    #[test]
    fn should_raise_error_if_response_does_not_follow_expected_pattern() {
        let data = "VALUE test 0 4".to_string();

        let result = parse_response(data).unwrap_err();
        assert_eq!(result.to_string(), "Response is invalid");
    }

    #[test]
    fn should_raise_error_if_response_not_contain_data_expected() {
        let data = "VALUE test 0 4\r\n".to_string();

        let result = parse_response(data).unwrap_err();
        assert_eq!(result.to_string(), "Response is invalid");
    }

		#[test]
    fn should_raise_error_if_separator_comes_first() {
        let data = "\r\nVALUE test 0 4".to_string();

        let result = parse_response(data).unwrap_err();
        assert_eq!(result.to_string(), "Response is invalid");
    }

		#[test]
    fn should_raise_error_if_separator_comes_first_and_in_last_position() {
        let data = "\r\nVALUE test 0 4\r\n".to_string();

        let result = parse_response(data).unwrap_err();
        assert_eq!(result.to_string(), "Response is invalid");
    }
}
