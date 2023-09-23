use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use crate::entities::prelude::Log;
use crate::entities::log;
use crate::entities::log::Model as LogModel;
use crate::log::Level;

#[allow(dead_code)]
pub async fn get_logs_by_date(db: &DatabaseConnection, date: chrono::NaiveDate, max: Option<i32>) -> Vec<LogModel> {

    let max = max.unwrap_or(100) as u64;

    let logs = match Log::find()
        .filter(log::Column::Date.eq(date))
        .limit(max)
        .all(db)
        .await
    {
        Ok(logs) => logs.into_iter().collect(),
        Err(e) => {
            error!("Error getting logs: {}", e);
            Vec::new()
        }
    };

    logs
}

#[allow(dead_code)]
pub async fn get_logs(db: &DatabaseConnection, max: Option<i32>) -> Vec<LogModel> {

     let max = max.unwrap_or(100) as u64;

     let logs = match Log::find()
         .limit(max)
         .all(db)
        .await
    {
        Ok(logs) => logs.into_iter().collect(),
        Err(e) => {
            error!("Error getting logs: {}", e);
            Vec::new()
        }
    };

    logs
}

#[allow(dead_code)]
pub async fn get_log_by_id(db: &DatabaseConnection, id: i32) -> Option<LogModel> {

    let log = match Log::find_by_id(id)
        .one(db)
        .await
    {
        Ok(log) => log,
        Err(e) => {
            error!("Error getting log: {}", e);
            return None;
        }
    };

    log
}

#[allow(dead_code)]
pub async fn get_logs_by_timeframe(db: &DatabaseConnection, start: chrono::NaiveDateTime, end: chrono::NaiveDateTime, max: Option<i32>) -> Vec<LogModel> {

    let max = max.unwrap_or(100) as u64;

    let logs = match Log::find()
        .filter(log::Column::Date.between(start, end))
        .limit(max)
        .all(db)
        .await
    {
        Ok(logs) => logs.into_iter().collect(),
        Err(e) => {
            error!("Error getting logs: {}", e);
            Vec::new()
        }
    };

    logs
}

#[allow(dead_code)]
pub async fn get_logs_by_level(db: &DatabaseConnection, level: Level, max: Option<i32>) -> Vec<LogModel> {

    let max = max.unwrap_or(100) as u64;

    let logs = match Log::find()
        .filter(log::Column::Level.eq(level.to_string().to_lowercase()))
        .limit(max)
        .all(db)
        .await
    {
        Ok(logs) => logs.into_iter().collect(),
        Err(e) => {
            error!("Error getting logs: {}", e);
            Vec::new()
        }
    };

    logs
}

#[allow(dead_code)]
pub async fn get_logs_by_user(db: &DatabaseConnection, user: String, max: Option<i32>) -> Vec<LogModel> {

    let max = max.unwrap_or(100) as u64;

    let logs = match Log::find()
        .filter(log::Column::User.eq(user))
        .limit(max)
        .all(db)
        .await
    {
        Ok(logs) => logs.into_iter().collect(),
        Err(e) => {
            error!("Error getting logs: {}", e);
            Vec::new()
        }
    };

    logs
}