mod routes;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use inertia_rust::actix::InertiaMiddleware;
use inertia_rust::template_resolvers::ViteHBSTemplateResolver;
use inertia_rust::{Inertia, InertiaConfig, InertiaVersion};
use routes::register_routes;
use std::env;
use vite_rust::{Vite, ViteConfig};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // initializes Vite
    let vite = Vite::new(
        ViteConfig::default()
            .set_manifest_path("public/bundle/manifest.json")
            .set_entrypoints(vec!["www/app.tsx"])
            .set_prefix("bundle")
            .set_heart_beat_retries_limit(2),
    )
    .await
    .unwrap();

    let version = vite.get_hash().unwrap_or("development").to_string();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let domain = env::var("DOMAIN").unwrap();
    let https = env::var("WITH_HTTPS").unwrap().parse::<bool>().unwrap();
    let app_url = format!(
        "{}://{}:{}",
        if https { "https" } else { "http" },
        domain,
        port
    )
    .leak();

    let template_reslver = ViteHBSTemplateResolver::builder()
        .set_template_path("www/root.hbs")
        .set_vite(vite)
        .build()
        .unwrap();

    let inertia_config = InertiaConfig::builder()
        .set_url(app_url)
        .set_version(InertiaVersion::Literal(version))
        .set_template_resolver(Box::new(template_reslver))
        .build();

    // initializes Inertia struct
    let inertia = Data::new(Inertia::new(inertia_config)?);

    println!("Starting the server at {}:{}.", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(inertia.clone())
            .configure(register_routes)
            .wrap(InertiaMiddleware::new())
            // serves vite assets from /assets path
            .service(actix_files::Files::new("/assets", "./public/bundle/assets").prefer_utf8(true))
            // serves public assets directly from / path
            // needs to be the last service because it's a wildcard
            .service(actix_files::Files::new("/", "./public/").prefer_utf8(true))
    })
    .bind((host, port))?
    .run()
    .await
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn failling_test() {
//         let bar = "foo";
//         assert_eq!("foo", bar);
//     }
// }
