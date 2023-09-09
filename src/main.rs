use std::env;
use actix_web::{App, HttpServer, guard, web, HttpRequest, Responder};
use actix_web::web::Redirect;

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

    if let Ok(allow_redirect) = env::var("REDIRECT_TO_HTTPS") {
        if allow_redirect.ne("") {
            cfg.default_service(web::route().to(https_redirect_handler));
        }
    }
}

async fn https_redirect_handler(request: HttpRequest) -> impl Responder {
    let target_url = format!(
        "https://{}{}",
        request.connection_info().host(),
        request.uri().path()
    );
    Redirect::to(target_url).permanent()
}