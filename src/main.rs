mod routes;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use inertia_rust::resolvers::basic_vite_resolver;
use inertia_rust::{Inertia, InertiaConfig, InertiaVersion};
use routes::register_routes;
use std::sync::OnceLock;
use vite_rust::{Vite, ViteConfig};

static VITE: OnceLock<Vite> = OnceLock::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initializes Vite
    let vite = match Vite::new(
        ViteConfig::default()
            .set_manifest_path("public/bundle/manifest.json")
            .set_entrypoints(vec!["www/app.tsx"])
            .set_heart_beat_retries_limit(2),
    )
    .await
    {
        Ok(vite) => vite,
        Err(err) => panic!("{}", err),
    };

    let vite = VITE.get_or_init(move || vite);

    let inertia_config: InertiaConfig<Vite, String> = InertiaConfig::builder()
        .set_url("http://localhost:3000")
        .set_version(InertiaVersion::Literal("inertia-version".to_string()))
        .set_template_path("www/root.html")
        .set_template_resolver(&basic_vite_resolver)
        .set_template_resolver_data(vite)
        .build();

    // initializes Inertia struct
    let inertia = Inertia::new(inertia_config)?;

    // stores Inertia as an AppData in a way that is not cloned for each worker
    let inertia = Data::new(inertia);
    let inertia_clone = Data::clone(&inertia);

    println!("Starting the server.");

    HttpServer::new(move || {
        App::new()
            .app_data(inertia_clone.clone())
            .configure(register_routes)
            // serves vite assets from /assets path
            .service(actix_files::Files::new("/assets", "./public/bundle/assets").prefer_utf8(true))
            // serves public assets directly from / path
            // needs to be the last service because it's a wildcard
            .service(actix_files::Files::new("/", "./public/").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
