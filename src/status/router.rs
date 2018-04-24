use actix_web::App;
use actix_web::HttpRequest;
use status::controller;

struct Route {
    path: String,
    action: fn(HttpRequest) -> String,
}

pub trait Routable {
    fn add_routes(self, app: App) -> App;
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        let prefix = "/status";
        Router {
            routes: vec![
                Route {
                    path: format!("{}", prefix),
                    action: controller::index,
                },
                Route {
                    path: prefix.to_string() + "/service", // .push_str("/{name}"),
                    action: controller::for_name,
                },
            ],
        }
    }
}

impl Routable for Router {
    fn add_routes(self, app: App) -> App {
        let mut res_app = app;
        for Route { path, action } in self.routes {
            res_app = res_app.resource(&path, move |r| r.f(action));
        }
        res_app
    }
}
