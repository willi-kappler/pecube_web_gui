// use std::fs::File;
// use std::io::Read;
// use std::io;

use hyper::{StatusCode, Body};
use mime;
use futures::{Stream, Future, future};

use gotham::state::{State, FromState};
use gotham::http::response::create_response;
use gotham::handler::{HandlerFuture, IntoHandlerError};

use helper;

pub fn handle_login(mut state: State) -> Box<HandlerFuture> {
    let handler_future = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();

                // println!("Body: {}", body_content);
                // Body: login=test&password=12345

                let post_parameters = helper::extract_post_params(&body_content);

                println!("post_parameters: {:?}", post_parameters);

                let page = "handle login".to_string();

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
