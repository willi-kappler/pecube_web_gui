use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io;



use nom::{IResult, alphanumeric};
use handlebars::{Handlebars, RenderError};


static SHOW_LOGIN_FILE : &str = "html/show_login.hbs";
static WELCOME_USER_FILE : &str = "html/welcome_user.hbs";


#[derive(Clone, Deserialize, Serialize, StateData, Default, Debug)]
pub struct UserData {
    pub login_id: String,
    pub logged_in: bool,
    pub last_login: i64,
}

impl UserData {
    pub fn new() -> Self {
        UserData{ login_id: "".to_string(), logged_in: false, last_login: 0 }
    }
}

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.login_id, self.logged_in, self.last_login)
    }
}

fn load_page(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut result = String::new();

    file.read_to_string(&mut result)?;

    Ok(result)
}

fn could_not_load_page(file_name: &str) -> String {
    format!("
        <html>
            <head>
                <title>Pecube Web GUI</title>
            </head>
            <body>
                <h1>Could not load page: '{}'</h1>
            </body>
        </html>", file_name)
}

fn handlears_render_error(error: RenderError) -> String {
    format!("
        <html>
            <head>
                <title>Pecube Web GUI</title>
            </head>
            <body>
                <h1>Could not render page: '{}'</h1>
            </body>
        </html>", error)
}

pub fn get_login_page(message: &str) -> String {
    let mut hb = Handlebars::new();

    match hb.register_template_file("show_login", SHOW_LOGIN_FILE) {
        Ok(_) => {
            match hb.render("show_login", &json!({"message": message})) {
                Ok(page) => page,
                Err(e) => handlears_render_error(e)
            }
        }
        Err(_) => {
            could_not_load_page(SHOW_LOGIN_FILE)
        }
    }

    // TODO: template engine, show message if not empty
    /*
    match load_page(SHOW_LOGIN_FILE) {
        Ok(result) => result,
        Err(_) => could_not_load_page(SHOW_LOGIN_FILE)
    }
    */
}

pub fn get_welcome_user_page(login_id: &str) -> String {
    let mut hb = Handlebars::new();

    match hb.register_template_file("welcome_user", WELCOME_USER_FILE) {
        Ok(_) => {
            match hb.render("welcome_user", &json!({"login_id": login_id})) {
                Ok(page) => page,
                Err(e) => handlears_render_error(e)
            }
        }
        Err(_) => {
            could_not_load_page(WELCOME_USER_FILE)
        }
    }


    // TODO: template engine, show login id of the user
    /*
    match load_page(WELCOME_USER_FILE) {
        Ok(result) => result,
        Err(_) => could_not_load_page(WELCOME_USER_FILE)
    }
    */
}

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
        IResult::Incomplete(_i) => {
            // println!("extract_post_params: error incomplete: {:?}", i);
        },
        IResult::Error(_e) => {
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
    fn test_parse_parameters3() {
        let result = parse_parameters(" login=test1&time=");
        let expected = IResult::Done("&time=", vec![
            ("login".to_string(), "test1".to_string())
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
