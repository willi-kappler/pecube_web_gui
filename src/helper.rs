use std::collections::HashMap;

use nom::{IResult, alphanumeric};

named!(parse_parameters<&str, Vec<(String, String)>>, do_parse!(
    first: complete!(ws!(parse_kv_tuple)) >>
    rest: many0!(ws!(parse_next_kv_tuple)) >>
    ({
        // println!("parse_parameters, first: {:?}", first);

        let mut result = Vec::new();
        result.push(first);
        result.extend(rest);
        result
    })
));

named!(parse_kv_tuple<&str, (String, String)>, do_parse!(
    first: alphanumeric >> tag!("=") >>
    second: alphanumeric >>
    ({
        // println!("parse_kv_tuple, first: {}, second: {}", first, second);

        (first.to_string(), second.to_string())
    })
));

named!(parse_next_kv_tuple<&str, (String, String)>, do_parse!(
    tag!("&") >>
    kv_tuple: complete!(parse_kv_tuple) >>
    ({
        // println!("parse_next_kv_tuple, kv_tuple: {:?}", kv_tuple);
        kv_tuple
    })
));

pub fn extract_post_params(message_body: &str) -> HashMap<String, String> {
    let mut post_parameters = HashMap::new();

    // println!("message_body: {}", message_body);

    match parse_parameters(message_body) {
        IResult::Done(_, result) => {
            // println!("extract_post_params: success");
            for (key, value) in result {
                post_parameters.insert(key, value);
            }
        },
        IResult::Incomplete(i) => {
            // println!("extract_post_params: error incomplete: {:?}", i);
        },
        IResult::Error(e) => {
            // println!("extract_post_params: error: {}", e);
        }
    }

    post_parameters
}


#[cfg(test)]
mod tests {
    use nom::{IResult};
    use std::collections::HashMap;

    use super::{parse_parameters, parse_kv_tuple, extract_post_params};

    #[test]
    fn test_parse_kv_tuple1() {
        let result = parse_kv_tuple("login=test1");
        let expected = IResult::Done("", ("login".to_string(), "test1".to_string()));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_parameters1() {
        let result = parse_parameters("login=test1");
        let expected = IResult::Done("", vec![
            ("login".to_string(), "test1".to_string())
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_parameters2() {
        let result = parse_parameters(" login=test1&time=12345");
        let expected = IResult::Done("", vec![
            ("login".to_string(), "test1".to_string()),
            ("time".to_string(), "12345".to_string())
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_post_params1() {
        let result = extract_post_params("    login=test1&time=12345");
        let mut expected = HashMap::new();

        expected.insert("login".to_string(), "test1".to_string());
        expected.insert("time".to_string(), "12345".to_string());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_post_params2() {
        let result = extract_post_params(" login=test&password=12345     ");
        let mut expected = HashMap::new();

        expected.insert("login".to_string(), "test".to_string());
        expected.insert("password".to_string(), "12345".to_string());

        assert_eq!(result, expected);
    }
}
