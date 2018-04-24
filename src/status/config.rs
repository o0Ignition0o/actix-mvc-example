use status::status::{for_name, index};
use actix_web::App;

pub fn configure(app: App) -> App {
    app.resource("/status", |r| r.get().f(index))
        .resource("/status/service", |r| r.get().f(for_name))
}
