use std::fs::File;
use std::io::Read;
use std::io;

use hyper::{Response, StatusCode};
use mime;

use gotham::state::{State, FromState};
use gotham::http::response::create_response;
use gotham::middleware::session::{SessionData};

use helper;

fn get_login_page(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut result = String::new();

    file.read_to_string(&mut result)?;

    Ok(result)
}

pub fn show_login(state: State) -> (State, Response) {
    println!("show_login");

    let user_data = {
        let user_data: &helper::UserData = SessionData::<helper::UserData>::borrow_from(&state);
        user_data.clone()
    };

    println!("user_data: {:?}", user_data);

    let page = if user_data.logged_in {
        let file_name = "html/welcome_user.html";

        format!("User {} already logged in", user_data.login_id)
    } else {
        let file_name = "html/show_login.html";

        match get_login_page(file_name) {
                Ok(result) => result,
                Err(_) => format!("Could not load page '{}'", file_name)
        }
    };

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((page.into_bytes(), mime::TEXT_HTML)),
    );

    (state, res)
}
