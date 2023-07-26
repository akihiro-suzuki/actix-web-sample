use actix_web::{get, post, web, HttpResponse};

use crate::{domains::error::Error, usecases::data::DataUsecase};

#[get("/data/dump")]
async fn index(uc: web::Data<DataUsecase>) -> Result<HttpResponse, Error> {
    uc.dump().await.map(|map| HttpResponse::Ok().json(map))
}

#[post("/data/clear")]
async fn clear(uc: web::Data<DataUsecase>) -> Result<HttpResponse, Error> {
    uc.clear().await.map(|_| HttpResponse::Ok().finish())
}
