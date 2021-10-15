use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub min_stock: i32,
    pub max_stock: Option<i32>,
    pub stock: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewItemRequest {
    pub name: String,
    pub min_stock: i32,
    pub max_stock: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemRequest {
    pub id: i32,
    pub name: String,
    pub min_stock: i32,
    pub max_stock: Option<i32>,
}
