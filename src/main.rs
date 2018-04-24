extern crate actix;
extern crate actix_web;

mod global;
mod status;
mod user;

use actix_web::{server, App};

use global::router::Routable;
use status::router::StatusRouter;
use user::router::UserRouter;

fn main() {
    server::new(move || {
        let mut app = App::new();
        let status_router = StatusRouter::new();
        let user_router = UserRouter::new();
        app = status_router.add_routes(app);
        app = user_router.add_routes(app);
        app
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
