use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::{self, Next},
    response::{Html, Response},
    routing::get,
    Router,
};

use std::net::SocketAddr;

use tower_http::services::{ServeDir, ServeFile};

use super::websocket;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let controller_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../controller/web/client");

    let index_path = controller_path.join("index.html");

    let controller_files = ServeDir::new(controller_path.clone())
        .not_found_service(ServeFile::new(index_path.clone()));

    let app = Router::new()
        // Página principal de ONA Core
        .route("/", get(index))
        // WebSocket del controlador
        .route("/ws", get(websocket::handler))
        // Archivos del controlador
        .nest_service("/controller", controller_files)
        .layer(middleware::from_fn(controller_no_cache));

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("======================================");
    println!("        ONA CORE HTTP SERVER");
    println!("======================================");

    println!("HTTP Server running at:");
    println!("http://localhost:8080");

    println!();

    println!("ONA Controller:");
    println!("http://localhost:8080/controller/");

    println!();

    println!("ONA WebSocket:");
    println!("ws://localhost:8080/ws");

    println!();

    println!("Waiting for mobile controllers...");

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(
        r#"
        <h1>ONA Gaming Studio</h1>

        <p>
            ONA Core HTTP Server is running.
        </p>

        <p>
            <a
                href="/controller/"
                style="color:#58b9ff;"
            >
                Open ONA Controller
            </a>
        </p>
        "#,
    )
}

async fn controller_no_cache(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, must-revalidate"),
    );
    response
        .headers_mut()
        .insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    response
        .headers_mut()
        .insert(header::EXPIRES, HeaderValue::from_static("0"));
    response
}
