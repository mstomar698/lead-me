pub mod github;
pub mod google;
pub mod twitter;

pub fn get_command_from_q_s(query_string: &str) -> &str {
    // Checks for white_space and return string before it
    // if only one word is parsed it will return it directly
    if query_string.contains(' ') {
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    &query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_q_s_no_whitespace() {
        let actual = get_command_from_q_s("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_q_s_with_whitespace() {
        let actual = get_command_from_q_s("tw @github");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
