use actix_web::App;
use actix_web::HttpRequest;

pub struct Route {
    pub path: String,
    pub action: fn(HttpRequest) -> String,
}

impl Route {
    pub fn new(path: String, action: fn(HttpRequest) -> String) -> Self {
        Route { path, action }
    }
}

pub trait Routable {
    fn add_routes(self, app: App) -> App;
}
