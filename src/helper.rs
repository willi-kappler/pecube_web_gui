use std::collections::HashMap;
use std::fmt;

use nom::{IResult, alphanumeric};
use handlebars::{Handlebars, RenderError};
use serde_json;

use hyper::{StatusCode, Body};
use mime;
use futures::{Stream, Future, future};
use gotham::state::{State, FromState};
use gotham::http::response::create_response;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::middleware::session::{SessionData};

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

fn load_hb_file(hb_name: &str, hb_file: &str, hb_json: &serde_json::Value) -> String {
    let mut hb = Handlebars::new();

    match hb.register_template_file(hb_name, hb_file) {
        Ok(_) => {
            match hb.render(hb_name, hb_json) {
                Ok(page) => page,
                Err(e) => handlears_render_error(e)
            }
        }
        Err(_) => {
            could_not_load_page(hb_file)
        }
    }
}

pub fn get_login_page(message: &str) -> String {
    load_hb_file("show_login", "html/show_login.hbs", &json!({"message": message}))
}

pub fn get_logout_page(user_id: &str) -> String {
    load_hb_file("logout", "html/logout.hbs", &json!({"user_id": user_id}))
}

pub fn get_welcome_user_page(login_id: &str) -> String {
    load_hb_file("welcome_user", "html/welcome_user.hbs", &json!({"login_id": login_id}))
}

pub fn handle_request<F: 'static>(mut state: State, handle_state: F) -> Box<HandlerFuture>
    where F:  Fn(&mut SessionData<UserData>, &HashMap<String, String>) -> String {
    println!("handle_request");

    let handler_future = Body::take_from(&mut state)
        .concat2()
        .then(move |full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();

                let post_parameters = extract_post_params(&body_content);

                let page = {
                    let mut session_data = SessionData::<UserData>::borrow_mut_from(&mut state);
                    handle_state(&mut session_data, &post_parameters)
                };

                let res = create_response(
                    &state,
                    StatusCode::Ok,
                    Some((page.into_bytes(), mime::TEXT_HTML)));
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(handler_future)
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
            println!("extract_post_params: error incomplete: {:?}", _i);
        },
        IResult::Error(_e) => {
            println!("extract_post_params: error: {}", _e);
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
