use axum::routing::{any_service, MethodRouter};
use tower_http::services::{ServeDir, ServeFile};

pub fn create() -> MethodRouter {
    any_service(handle_web_folder().not_found_service(handle_404()))
}

fn handle_404() -> ServeFile {
    tracing::info!("{:-<15} |> handle_404 |> {}", "SERVING_STATIC ", file!());

    ServeFile::new("../front-end/404.html")
}

fn handle_web_folder() -> ServeDir {
    tracing::info!(
        "{:-<15} |> handle_web_folder |> {}",
        "SERVING_STATIC ",
        file!()
    );

    ServeDir::new("../front-end/dist")
}
