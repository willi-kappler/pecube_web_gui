extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
extern crate chrono;
extern crate handlebars;


#[macro_use] extern crate gotham_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate nom;


use gotham::router::Router;
use gotham::router::builder::*;
use gotham::middleware::session::{NewSessionMiddleware};
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::new_pipeline;



mod show_login;
mod handle_login;
mod handle_logout;
mod configuration;
mod helper;


fn router() -> Router {

    let middleware = NewSessionMiddleware::default()
        .with_session_type::<helper::UserData>()
        .insecure();


    let (chain, pipelines) = single_pipeline(new_pipeline().add(middleware).build());

    build_router(chain, pipelines, |route| {
        route.get("/").to(show_login::show_login);
        route.post("/handle_login").to(handle_login::handle_login);
        route.post("/handle_logout").to(handle_logout::handle_logout);
    })
}

pub fn main() {
    let addr = "0.0.0.0:3030";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router())
}
