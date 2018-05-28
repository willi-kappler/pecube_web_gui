// use std::collections::HashMap;

use hyper::{StatusCode, Body};
use mime;
use futures::{Stream, Future, future};
// use chrono::{Local};

use gotham::state::{State, FromState};
use gotham::http::response::create_response;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::middleware::session::{SessionData};

use helper;

pub fn handle_logout(mut state: State) -> Box<HandlerFuture> {
    println!("handle_login");

    let handler_future = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();

                // let post_parameters = helper::extract_post_params(&body_content);

                let mut user_data = helper::UserData::new();

                {
                    let session_data = SessionData::<helper::UserData>::borrow_mut_from(&mut state);
                    user_data.login_id = session_data.login_id.clone();
                    user_data.logged_in = session_data.logged_in.clone();
                    user_data.last_login = session_data.last_login.clone();

                    session_data.logged_in = false;
                    session_data.login_id = "".to_string();
                    session_data.last_login = 0;
                }

                println!("handle_out, user_data old: {}", user_data);

                let page = helper::get_logout_page(&user_data.login_id);

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
