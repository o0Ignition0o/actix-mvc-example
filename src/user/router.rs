use actix_web::App;
use user::controller;
use global::router::{Routable, Route};

pub struct UserRouter {
    routes: Vec<Route>,
}

impl UserRouter {
    pub fn new() -> Self {
        let prefix = "/user";
        UserRouter {
            routes: vec![Route::new(format!("{}", prefix), controller::count)],
        }
    }
}

impl Routable for UserRouter {
    fn add_routes(self, app: App) -> App {
        let mut res_app = app;
        for Route { path, action } in self.routes {
            res_app = res_app.resource(&path, move |r| r.f(action));
        }
        res_app
    }
}
