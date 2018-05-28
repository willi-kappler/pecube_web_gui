use std::collections::HashMap;

use chrono::{Local};

use gotham::state::{State};
use gotham::handler::{HandlerFuture};

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

pub fn handle_login(state: State) -> Box<HandlerFuture> {
    println!("handle_login");

    helper::handle_request(state, |session_data, post_parameters| {
        let mut user_data = helper::UserData::new();

        user_data.login_id = session_data.login_id.clone();
        user_data.logged_in = session_data.logged_in.clone();
        user_data.last_login = session_data.last_login.clone();

        println!("handle_login, user_data old: {}", user_data);

        if user_data.logged_in {
            helper::get_welcome_user_page(&user_data.login_id)
        } else if check_login(&post_parameters) {
            user_data.logged_in = true;
            user_data.login_id = post_parameters["login"].clone();
            user_data.last_login = Local::now().timestamp();

            println!("handle_login, user_data new: {}", user_data);

            session_data.logged_in = true;
            session_data.login_id = user_data.login_id.clone();
            session_data.last_login = user_data.last_login;

            helper::get_welcome_user_page(&user_data.login_id)
        } else {
            helper::get_login_page("Login failed, wrong user data")
        }
    })
}
