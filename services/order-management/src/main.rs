use actix_web::{post, web, App, HttpResponse, HttpServer};
use models::{Inventory, Order, OrderItem};
use mongodb::{bson::doc, Client, Collection};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use reqwest::Client as HttpClient;
use serde_json::json;
use std::env;
use std::time::Duration;

struct AppState {
    orders: Collection<Order>,
    producer: FutureProducer,
    http_client: HttpClient,
    inventory_api: String,
}

#[post("/orders")]
async fn create_order(order: web::Json<Order>, state: web::Data<AppState>) -> HttpResponse {
    let mut order = order.into_inner();
    order.order_id = uuid::Uuid::new_v4();

    for item in &order.items {
        let inv: Inventory = state
            .http_client
            .get(format!("{}/inventory/{}", state.inventory_api, item.sku))
            .send()
            .await
            .map_err(|e| format!("Inventory fetch error: {}", e))
            .unwrap()
            .json()
            .await
            .map_err(|e| format!("Inventory parse error: {}", e))
            .unwrap();

        if inv.quantity < item.quantity {
            return HttpResponse::BadRequest()
                .json(format!("Insufficient stock for SKU {}", item.sku));
        }

        let update = json!({ "quantity": inv.quantity - item.quantity });
        state
            .http_client
            .patch(format!("{}/inventory/{}", state.inventory_api, item.sku))
            .json(&update)
            .send()
            .await
            .map_err(|e| format!("Inventory update error: {}", e))
            .unwrap();
    }

    let result = state.orders.insert_one(&order, None).await;
    match result {
        Ok(_) => {
            let event = json!({
                "event": "order.created",
                "order_id": order.order_id,
                "total": order.total,
                "items": order.items
            });
            let event = event.to_string();
            let order_id = order.order_id.to_string();
            let record = FutureRecord::to("order.created")
                .payload(&event)
                .key(&order_id);
            match state.producer.send(record, Duration::from_secs(0)).await {
                Ok(_) => HttpResponse::Ok().json(&order),
                Err((e, _)) => {
                    HttpResponse::InternalServerError().json(format!("Kafka error: {}", e))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("DB error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());
    let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let inventory_api = env::var("INVENTORY_API").unwrap_or("http://localhost:8082".to_string());

    let client = Client::with_uri_str(&mongodb_uri).await.unwrap();
    let db = client.database("ecommerce");
    let orders = db.collection::<Order>("orders");
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .create()
        .expect("Producer creation failed");
    let http_client = HttpClient::new();

    let state = web::Data::new(AppState {
        orders,
        producer,
        http_client,
        inventory_api,
    });

    HttpServer::new(move || App::new().app_data(state.clone()).service(create_order))
        .bind(("0.0.0.0", 8083))?
        .run()
        .await
}
