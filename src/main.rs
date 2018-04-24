extern crate actix;
extern crate actix_web;

mod status;

use status::router::{Routable, Router};

use actix_web::{server, App};

fn main() {
    server::new(move || {
        let app = App::new();
        let status_router = Router::new();
        status_router.add_routes(app)
    }).bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
