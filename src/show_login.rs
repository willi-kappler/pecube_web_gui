use hyper::{Response, StatusCode};
use mime;

use gotham::state::{State, FromState};
use gotham::http::response::create_response;
use gotham::middleware::session::{SessionData};

use helper;

pub fn show_login(state: State) -> (State, Response) {
    println!("show_login");

    let user_data = {
        let user_data: &helper::UserData = SessionData::<helper::UserData>::borrow_from(&state);
        user_data.clone()
    };

    println!("show_login, user_data: {:?}", user_data);

    let page = if user_data.logged_in {
        helper::get_welcome_user_page(&user_data.login_id)
    } else {
        helper::get_login_page("")
    };

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((page.into_bytes(), mime::TEXT_HTML)),
    );

    (state, res)
}
