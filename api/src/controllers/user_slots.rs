use actix_web::{get, post, web, HttpResponse};
use itertools::Itertools;

use crate::{controllers::time_helper, usecases::user_slots::UserSlotUsecase};

#[derive(Debug, serde::Deserialize)]
struct UserSlotParams {
    accounts: String,
    #[serde(rename = "startTime")]
    start_time: String,
    #[serde(rename = "endTime")]
    end_time: String,
}
#[get("/slots")]
async fn index(
    uc: web::Data<UserSlotUsecase>,
    query_params: web::Query<UserSlotParams>,
) -> Result<HttpResponse, actix_web::Error> {
    // MEMO: 日付に変換できない場合は考慮してない
    let accounts = query_params
        .accounts
        .split(',')
        .map(|x| x.to_string())
        .collect_vec();
    let start_date = time_helper::to_naive_datetime(query_params.start_time.as_str());
    let end_date = time_helper::to_naive_datetime(query_params.end_time.as_str());
    let slots = uc
        .fetch_confirmable_slots(&accounts, start_date, end_date)
        .await?
        .iter()
        .map(|x| time_helper::to_ymdhm_str(&x.start_date))
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(slots))
}

#[derive(Debug, serde::Deserialize)]
struct ConfirmSlotParam {
    accounts: Vec<String>,
    #[serde(rename = "startTime")]
    start_time: String,
}
#[post("/confirm")]
async fn post(
    uc: web::Data<UserSlotUsecase>,
    params: web::Json<ConfirmSlotParam>,
) -> Result<HttpResponse, actix_web::Error> {
    let start_time = time_helper::to_naive_datetime(params.start_time.as_str());
    uc.confirm_users_slot(&params.accounts, start_time).await?;
    Ok(HttpResponse::Created().finish())
}
