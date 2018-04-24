extern crate actix;
extern crate actix_web;

mod status;
mod user;

use actix_web::{server, App};

use status::config as status_config;
use user::config as user_config;

fn main() {
    server::new(move || {
        App::new()
            .configure(user_config::configure)
            .configure(status_config::configure)
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
