use actix_web::App;
use status::controller;
use global::router::{Routable, Route};

pub struct StatusRouter {
    routes: Vec<Route>,
}

impl StatusRouter {
    pub fn new() -> Self {
        let prefix = "/status";
        StatusRouter {
            routes: vec![
                Route::new(format!("{}", prefix), controller::index),
                Route::new(prefix.to_string() + "/service", controller::for_name),
            ],
        }
    }
}

impl Routable for StatusRouter {
    fn add_routes(self, app: App) -> App {
        let mut res_app = app;
        for Route { path, action } in self.routes {
            res_app = res_app.resource(&path, move |r| r.f(action));
        }
        res_app
    }
}
