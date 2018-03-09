use std::collections::HashMap;

use hyper::{StatusCode, Body};
use mime;
use futures::{Stream, Future, future};
use chrono::{Local};

use gotham::state::{State, FromState};
use gotham::http::response::create_response;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::middleware::session::{SessionData};

use helper;

fn check_login(post_parameters: &HashMap<String, String>) -> bool {
    if post_parameters.contains_key("login") {
        if post_parameters.contains_key("password") {
            let login = &post_parameters["login"];
            let password = &post_parameters["password"];

            if login == "test1" && password == "1234567890" { return true }

        }
    }

    false
}

pub fn handle_login(mut state: State) -> Box<HandlerFuture> {
    println!("handle_login");

    let handler_future = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();

                let post_parameters = helper::extract_post_params(&body_content);

                let mut user_data = helper::UserData::new();

                {
                    let session_data = SessionData::<helper::UserData>::borrow_from(&state);
                    user_data.login_id = session_data.login_id.clone();
                    user_data.logged_in = session_data.logged_in.clone();
                    user_data.last_login = session_data.last_login.clone();
                }

                println!("handle_login, user_data old: {}", user_data);

                let page = if user_data.logged_in {
                    helper::get_welcome_user_page(&user_data.login_id)
                } else if check_login(&post_parameters) {
                    user_data.logged_in = true;
                    user_data.login_id = post_parameters["login"].clone();
                    user_data.last_login = Local::now().timestamp();

                    println!("handle_login, user_data new: {}", user_data);

                    {
                        let session_data = SessionData::<helper::UserData>::borrow_mut_from(&mut state);
                        session_data.logged_in = true;
                        session_data.login_id = user_data.login_id.clone();
                        session_data.last_login = user_data.last_login;
                    }

                    helper::get_welcome_user_page(&user_data.login_id)
                } else {
                    helper::get_login_page("Login failed, wrong user data")
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
