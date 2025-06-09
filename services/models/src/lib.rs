use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sku: String,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub order_id: Uuid,
    pub items: Vec<OrderItem>,
    pub total: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItem {
    pub sku: String,
    pub quantity: i32,
    pub price: f64,
}