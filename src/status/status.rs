use actix_web::HttpRequest;

pub fn index(_: HttpRequest) -> String {
    "Status is OK. Server is ready to go.".into()
}

pub fn for_name(request: HttpRequest) -> String {
    let service_name: &str = request.query().get("name").unwrap_or("undefined").into();
    format!(
        "Status for service {} is Ok, everything is fine ! :)",
        service_name
    )
}
