use std::fs::File;
use std::io::Read;
use std::io;

use hyper::{Response, StatusCode};
use mime;

use gotham::state::State;
use gotham::http::response::create_response;

fn get_login_page(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut result = String::new();

    file.read_to_string(&mut result)?;

    Ok(result)
}

pub fn show_login(state: State) -> (State, Response) {
    let file_name = "html/show_login.html";

    let page = match get_login_page(file_name) {
            Ok(result) => result,
            Err(_) => format!("Could not load page '{}'", file_name)
        };

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((page.into_bytes(), mime::TEXT_HTML)),
    );

    (state, res)
}
