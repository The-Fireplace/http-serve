use std::env;
use actix_web::{App, HttpServer, guard, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .configure(configure)
    });

    server
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let mount_path = env::var("MOUNT_PATH");
    let serve_from = env::var("SERVE_FROM");
    if let Ok(mount_path) = mount_path {
        if let Ok(serve_from) = serve_from {
            cfg.service(actix_files::Files::new(&mount_path, &serve_from)
                .guard(guard::Get())
                .show_files_listing()
            );
        }
    }
}