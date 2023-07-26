use std::{env, sync::Arc};

use actix_web::{middleware::Logger, web, App, HttpServer};
use controllers::{data, user_slots};
use sqlx::mysql::MySqlPoolOptions;
use usecases::{data::DataUsecase, user_slots::UserSlotUsecase};
mod controllers;
mod domains;
mod sql_clients;
mod usecases;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // FIXME: 今回は無条件でdev.envの内容を読み込む
    dotenvy::from_filename_override("dev.env").ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let pool = Arc::new(
        MySqlPoolOptions::default()
            .connect(db_url.as_str())
            .await
            .unwrap(),
    );
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::from(Arc::new(DataUsecase::new(pool.clone()))))
            .app_data(web::Data::from(Arc::new(UserSlotUsecase::new(
                pool.clone(),
            ))))
            .service(data::index)
            .service(data::clear)
            .service(user_slots::index)
            .service(user_slots::post)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
