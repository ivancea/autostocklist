use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastQuery {
    pub history_days: Option<i64>,
    pub forecast_days: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsagePoint {
    pub date: NaiveDate,
    pub usage: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPoint {
    pub date: NaiveDate,
    pub usage: f64,
}

#[derive(Serialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ForecastStrategy {
    WeekdaySeasonality,
    SimpleAverage,
    NoData,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastResult {
    pub item_id: i32,
    pub current_stock: i32,
    pub min_stock: i32,
    pub history_start: NaiveDate,
    pub history_end: NaiveDate,
    pub strategy: ForecastStrategy,
    pub history_days: i64,
    pub non_zero_days: i64,
    pub average_daily_usage: f64,
    pub history: Vec<UsagePoint>,
    pub forecast: Vec<ForecastPoint>,
    pub projected_days_to_min: Option<i32>,
    pub projected_min_date: Option<NaiveDate>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastSummary {
    pub item_id: i32,
    pub name: String,
    pub current_stock: i32,
    pub min_stock: i32,
    pub strategy: ForecastStrategy,
    pub average_daily_usage: f64,
    pub projected_days_to_min: Option<i32>,
    pub projected_min_date: Option<NaiveDate>,
}
