use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Настройки маршрутизации: если зашли на "/", вызываем функцию root
    let app = Router::new().route("/", get(root));

    // Запускаем сервер на 3000 порту
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Сервер запущен на http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Привет, мир! Это мой сайт на Rust!"
}
