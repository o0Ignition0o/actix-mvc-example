use actix_web::HttpRequest;

#[allow(dead_code)]
struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        User { name, age }
    }
}

pub fn count(_: HttpRequest) -> String {
    let users = vec![User::new("Jeremy".into(), 28), User::new("John".into(), 16)];

    format!("Lots of users right here ! {} total.", users.len())
}
