use actix_web::{patch, web, App, HttpResponse, HttpServer};
use mongodb::{bson::doc, Client, Collection};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde_json::{self, Value};
use models::Inventory;
use tokio::spawn;
use std::env;

struct AppState {
    inventory: Collection<Inventory>,
}

#[derive(serde::Deserialize)]
struct UpdateStock {
    quantity: i32,
}

#[patch("/inventory/{sku}")]
async fn update_stock(
    path: web::Path<String>,
    update: web::Json<UpdateStock>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let sku = path.into_inner();
    let result = state.inventory
        .update_one(
            doc! { "sku": &sku },
            doc! { "$set": { "quantity": update.quantity } },
            None,
        )
        .await;
    match result {
        Ok(res) if res.matched_count > 0 => HttpResponse::Ok().json("Stock updated"),
        Ok(_) => HttpResponse::NotFound().json("SKU not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("DB error: {}", e)),
    }
}

async fn consume_products(inventory: Collection<Inventory>) {
    let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .set("group.id", "inventory-consumer")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["product.created"]).expect("Subscription failed");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    let event: Value = serde_json::from_slice(payload).unwrap();
                    let sku = event["sku"].as_str().unwrap().to_string();
                    let inv = Inventory {
                        id: None,
                        sku,
                        quantity: 100,
                    };
                    inventory.insert_one(inv, None).await.unwrap();
                }
            }
            Err(e) => println!("Kafka error: {}", e),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(&mongodb_uri).await.unwrap();
    let db = client.database("ecommerce");
    let inventory = db.collection::<Inventory>("inventory");

    let inventory_clone = inventory.clone();
    spawn(async move { consume_products(inventory_clone).await });

    let state = web::Data::new(AppState { inventory });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(update_stock)
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await
}