use chrono::{Datelike, Duration, NaiveDate, Utc};

use crate::{
    database::{forecast::DailyUsage, Database},
    dtos::{
        forecast_dtos::{ForecastPoint, ForecastResult, ForecastStrategy, ForecastSummary, UsagePoint},
        item_dtos::Item,
    },
};

use super::error::ServiceError;

const DEFAULT_HISTORY_DAYS: i64 = 90;
const DEFAULT_FORECAST_DAYS: i64 = 30;
const MIN_HISTORY_DAYS_FOR_SEASONALITY: i64 = 14;
const MAX_DAYS_TO_MIN_FORECAST: i64 = 365;

#[derive(Clone, Copy)]
pub struct ForecastConfig {
    pub history_days: i64,
    pub forecast_days: i64,
}

impl ForecastConfig {
    pub fn from_query(
        history_days: Option<i64>,
        forecast_days: Option<i64>,
    ) -> Result<ForecastConfig, ServiceError> {
        let history_days = history_days.unwrap_or(DEFAULT_HISTORY_DAYS);
        let forecast_days = forecast_days.unwrap_or(DEFAULT_FORECAST_DAYS);

        if history_days <= 0 {
            return Err(ServiceError::Input(
                "historyDays must be greater than 0".to_owned(),
            ));
        }

        if forecast_days <= 0 {
            return Err(ServiceError::Input(
                "forecastDays must be greater than 0".to_owned(),
            ));
        }

        Ok(ForecastConfig {
            history_days,
            forecast_days,
        })
    }
}

#[derive(Clone)]
pub struct ForecastService {
    database: Database,
}

impl ForecastService {
    pub fn new(database: Database) -> ForecastService {
        ForecastService { database: database }
    }

    pub async fn forecast_item(
        &self,
        item_id: i32,
        config: ForecastConfig,
    ) -> Result<ForecastResult, ServiceError> {
        let item = self.database.get_item(item_id).await?;
        let (history_start, history_end, history) =
            self.load_history(item_id, config.history_days).await?;

        let computation = compute_forecast(&item, &history, history_end);

        let forecast_start = history_end + Duration::days(1);
        let forecast = build_forecast_series(
            forecast_start,
            config.forecast_days,
            &computation.weekday_usage,
        );

        Ok(ForecastResult {
            item_id: item.id,
            current_stock: item.stock,
            min_stock: item.min_stock,
            history_start,
            history_end,
            strategy: computation.strategy,
            history_days: history.len() as i64,
            non_zero_days: computation.non_zero_days,
            average_daily_usage: computation.average_daily_usage,
            history: history
                .into_iter()
                .map(|entry| UsagePoint {
                    date: entry.date,
                    usage: entry.usage,
                })
                .collect(),
            forecast,
            projected_days_to_min: computation.projected_days_to_min,
            projected_min_date: computation.projected_min_date,
        })
    }

    pub async fn forecast_all(
        &self,
        config: ForecastConfig,
    ) -> Result<Vec<ForecastSummary>, ServiceError> {
        let items = self.database.get_items().await?;
        let mut summaries = Vec::with_capacity(items.len());

        for item in items {
            let (_, history_end, history) =
                self.load_history(item.id, config.history_days).await?;
            let computation = compute_forecast(&item, &history, history_end);

            summaries.push(ForecastSummary {
                item_id: item.id,
                name: item.name,
                current_stock: item.stock,
                min_stock: item.min_stock,
                strategy: computation.strategy,
                average_daily_usage: computation.average_daily_usage,
                projected_days_to_min: computation.projected_days_to_min,
                projected_min_date: computation.projected_min_date,
            });
        }

        Ok(summaries)
    }

    async fn load_history(
        &self,
        item_id: i32,
        history_days: i64,
    ) -> Result<(NaiveDate, NaiveDate, Vec<DailyUsage>), ServiceError> {
        let history_end = Utc::now().naive_utc().date();
        let history_start = history_end - Duration::days(history_days - 1);

        let history = self
            .database
            .get_item_usage_series(item_id, history_start, history_end)
            .await?;

        Ok((history_start, history_end, history))
    }
}

struct ForecastComputation {
    strategy: ForecastStrategy,
    weekday_usage: [f64; 7],
    average_daily_usage: f64,
    non_zero_days: i64,
    projected_days_to_min: Option<i32>,
    projected_min_date: Option<NaiveDate>,
}

fn compute_forecast(item: &Item, history: &[DailyUsage], history_end: NaiveDate) -> ForecastComputation {
    let non_zero_days = history.iter().filter(|entry| entry.usage > 0).count() as i64;
    let strategy = select_strategy(history.len() as i64, non_zero_days);
    let weekday_usage = build_weekday_usage(history, strategy);
    let average_daily_usage = weekday_usage.iter().sum::<f64>() / 7.0;
    let (projected_days_to_min, projected_min_date) =
        project_min_date(item.stock, item.min_stock, history_end, &weekday_usage, average_daily_usage);

    ForecastComputation {
        strategy,
        weekday_usage,
        average_daily_usage,
        non_zero_days,
        projected_days_to_min,
        projected_min_date,
    }
}

fn select_strategy(history_days: i64, non_zero_days: i64) -> ForecastStrategy {
    if non_zero_days == 0 {
        ForecastStrategy::NoData
    } else if history_days >= MIN_HISTORY_DAYS_FOR_SEASONALITY {
        ForecastStrategy::WeekdaySeasonality
    } else {
        ForecastStrategy::SimpleAverage
    }
}

fn build_weekday_usage(history: &[DailyUsage], strategy: ForecastStrategy) -> [f64; 7] {
    let overall_average = if history.is_empty() {
        0.0
    } else {
        history.iter().map(|entry| entry.usage as f64).sum::<f64>() / history.len() as f64
    };

    match strategy {
        ForecastStrategy::WeekdaySeasonality => {
            let mut totals = [0.0; 7];
            let mut counts = [0u32; 7];

            for entry in history {
                let index = weekday_index(entry.date);
                totals[index] += entry.usage as f64;
                counts[index] += 1;
            }

            for index in 0..7 {
                totals[index] = if counts[index] == 0 {
                    overall_average
                } else {
                    totals[index] / counts[index] as f64
                };
            }

            totals
        }
        ForecastStrategy::SimpleAverage => [overall_average; 7],
        ForecastStrategy::NoData => [0.0; 7],
    }
}

fn build_forecast_series(
    start_date: NaiveDate,
    forecast_days: i64,
    weekday_usage: &[f64; 7],
) -> Vec<ForecastPoint> {
    let mut forecast = Vec::with_capacity(forecast_days as usize);

    for offset in 0..forecast_days {
        let date = start_date + Duration::days(offset);
        let usage = weekday_usage[weekday_index(date)];

        forecast.push(ForecastPoint { date, usage });
    }

    forecast
}

fn project_min_date(
    current_stock: i32,
    min_stock: i32,
    start_date: NaiveDate,
    weekday_usage: &[f64; 7],
    average_daily_usage: f64,
) -> (Option<i32>, Option<NaiveDate>) {
    if current_stock <= min_stock {
        return (Some(0), Some(start_date));
    }

    if average_daily_usage <= 0.0 {
        return (None, None);
    }

    let mut projected_stock = current_stock as f64;

    for day_offset in 1..=MAX_DAYS_TO_MIN_FORECAST {
        let date = start_date + Duration::days(day_offset);
        let usage = weekday_usage[weekday_index(date)];

        if usage > 0.0 {
            projected_stock -= usage;
        }

        if projected_stock <= min_stock as f64 {
            return (Some(day_offset as i32), Some(date));
        }
    }

    (None, None)
}

fn weekday_index(date: NaiveDate) -> usize {
    date.weekday().num_days_from_monday() as usize
}
