use user::user::{count, index};
use actix_web::App;

pub fn configure(app: App) -> App {
    app.resource("/user/", |r| r.get().f(index))
        .resource("/user/count", |r| r.get().f(count))
}
