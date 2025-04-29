use actix_web::{get, web::ServiceConfig, HttpRequest, Responder};
use inertia_rust::{
    hashmap, prop_resolver, Inertia, InertiaFacade, InertiaProp, IntoInertiaPropResult,
};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    Inertia::render_with_props(
        &req,
        "Index".into(),
        hashmap![ "message" => InertiaProp::lazy(prop_resolver!({ "Hello World".into_inertia_value() })) ],
    )
    .await
}

#[get("/hello")]
async fn hello_world() -> impl Responder {
    "Another message"
}

#[get("/foo")]
async fn foo(req: HttpRequest) -> impl Responder {
    Inertia::render(&req, "Foo".into()).await
}

pub fn register_routes(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(hello_world);
    cfg.service(foo);
}
