use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::{self, Key},
    middleware::Logger,
    web, App, HttpRequest, HttpServer, Result,
};

/// セッション付きのindexハンドラー
async fn index(session: Session, req: HttpRequest) -> Result<&'static str> {
    log::info!("{req:?}");

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        log::info!("SESSION value: {count}");
        counter = count + 1;
        session.insert("counter", counter)?;
    } else {
        session.insert("counter", counter)?;
    }

    Ok("セッションゲットだぜ！")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // ロガーを有効化
            .wrap(Logger::default())
            // クッキーセッションミドルウェアの有効化
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    // クッキーとセッションの有効期限をカスタマイズ
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}