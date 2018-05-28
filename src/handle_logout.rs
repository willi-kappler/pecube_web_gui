use gotham::state::{State};
use gotham::handler::{HandlerFuture};

use helper;

pub fn handle_logout(state: State) -> Box<HandlerFuture> {
    println!("handle_logout");

    helper::handle_request(state, |session_data, _post_parameters| {
        let mut user_data = helper::UserData::new();

        user_data.login_id = session_data.login_id.clone();
        user_data.logged_in = session_data.logged_in.clone();
        user_data.last_login = session_data.last_login.clone();

        session_data.logged_in = false;
        session_data.login_id = "".to_string();
        session_data.last_login = 0;

        println!("handle_out, user_data old: {}", user_data);

        if user_data.logged_in {
            helper::get_logout_page(&user_data.login_id)
        } else {
            helper::get_login_page("")
        }
    })
}
