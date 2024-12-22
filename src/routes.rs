use std::{collections::HashMap, sync::Arc};

use actix_web::{get, web::ServiceConfig, HttpRequest, Responder};
use inertia_rust::{actix::render_with_props, InertiaProp};
use vite_rust::Vite;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let mut props = HashMap::new();
    props.insert(
        "message".to_string(),
        InertiaProp::Lazy(Arc::new(|| {
            serde_json::Value::String("Hello World".to_string())
        })),
    );

    render_with_props::<Vite>(&req, "Index".into(), props).await
}

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Hello World"
}

pub fn register_routes(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(hello_world);
}
