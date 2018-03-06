use std::fs::File;
use std::io::Read;
use std::io;

use hyper::{Response, StatusCode};
use mime;

use gotham::state::State;
use gotham::http::response::create_response;

pub fn handle_login(state: State) -> (State, Response) {
    let page = "handle login".to_string();

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((page.into_bytes(), mime::TEXT_HTML)),
    );

    (state, res)
}
