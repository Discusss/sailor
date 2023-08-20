use rocket::http::Status;
use rocket::serde::json::{Json, json, Value};
use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};
use crate::entities::domains;
use crate::entities::prelude::{Blacklist, Domains};
use crate::utils::response::DataResponse;

#[get("/")]
pub async fn get_stats(db: &State<DatabaseConnection>) -> Result<Json<DataResponse>, Status> {
    let db = db as &DatabaseConnection;

    let total_domains = match Domains::find()
        .count(db)
        .await
    {
        Ok(total_domains) => total_domains,
        Err(_) => 0,
    };

    let blacklisted_count = match Blacklist::find()
        .count(db)
        .await
    {
        Ok(blacklisted_count) => blacklisted_count,
        Err(_) => 0,
    };

    let top_5_domains = match Domains::find()
        .order_by_asc(domains::Column::TimesConsulted)
        .limit(5)
        .all(db)
        .await
    {
        Ok(top_5_domains) => top_5_domains,
        Err(_) => vec![],
    };

    let top_5_domains: Vec<Value> = top_5_domains.into_iter().map(|domain| {
            json!({
                    "domain": domain.domain,
                    "times_consulted": domain.times_consulted,
                })
    }).collect();

    let response_json = DataResponse {
        status: "200".to_string(),
        data: json!({
            "total_domains": total_domains as i64,
            "blacklisted_count": blacklisted_count as i64,
            "top_5_domains": top_5_domains,
        })
    };
    Ok(Json(response_json))

}