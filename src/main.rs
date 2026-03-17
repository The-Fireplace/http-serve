use actix_web::web::Redirect;
use actix_web::{guard, web, App, HttpRequest, HttpServer, Responder};
use std::env;

const DEFAULT_MAX_URI_CHARS: usize = 2048;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || App::new().configure(configure));

    server.bind("0.0.0.0:8080")?.run().await
}

fn configure(cfg: &mut web::ServiceConfig) {
    let mount_path = env::var("MOUNT_PATH");
    let serve_from = env::var("SERVE_FROM");
    if let Ok(mount_path) = mount_path
        && let Ok(serve_from) = serve_from
    {
        cfg.service(
            actix_files::Files::new(&mount_path, serve_from)
                .guard(guard::Get())
                .show_files_listing()
                .prefer_utf8(true),
        );
    }

    if let Ok(https_redirect_host) = env::var("HTTPS_REDIRECT_HOST")
        && !https_redirect_host.is_empty()
    {
        let max_uri_length = env::var("MAX_URI_CHARACTERS").map(
            |max_uri_length| max_uri_length.parse::<usize>().unwrap_or(DEFAULT_MAX_URI_CHARS)
        ).unwrap_or(DEFAULT_MAX_URI_CHARS);
        cfg.default_service(web::route().to(move |request|
            https_redirect_handler(request, max_uri_length)
        ));
    }
}

async fn https_redirect_handler(
    request: HttpRequest,
    max_uri_length: usize,
) -> actix_web::Result<impl Responder> {
    let host = match env::var("HTTPS_REDIRECT_HOST") {
        Ok(host) => host,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("HTTPS_REDIRECT_HOST not set")),
    };
    let path = request.uri().path();
    if 8 + host.chars().count() + path.chars().count() > max_uri_length {
        return Err(actix_web::error::ErrorBadRequest("URI too long"));
    }
    let target_url = format!(
        "https://{}{}",
        host,
        path
    );
    Ok(Redirect::to(target_url).permanent())
}
