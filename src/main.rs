extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate gotham_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate nom;



use gotham::router::Router;
use gotham::router::builder::*;


mod show_login;
mod handle_login;
mod configuration;
mod helper;


fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(show_login::show_login);
        route.post("/handle_login").to(handle_login::handle_login);
    })
}

pub fn main() {
    let addr = "0.0.0.0:3030";
    println!("Listening for requests at http://{}", addr);

    // All incoming requests are delegated to the router for further analysis and dispatch
    gotham::start(addr, router())
}
