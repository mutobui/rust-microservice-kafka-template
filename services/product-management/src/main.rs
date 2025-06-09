use actix_web::{post, get, web, App, HttpResponse, HttpServer};
use mongodb::{bson::doc, Client, Collection};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;
use std::time::Duration;
use models::Product;
use uuid::Uuid;
use std::env;

struct AppState {
    products: Collection<Product>,
    producer: FutureProducer,
}

#[post("/products")]
async fn create_product(
    product: web::Json<Product>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let mut product = product.into_inner();
    product.product_id = Uuid::new_v4();
    let result = state.products.insert_one(&product, None).await;
    match result {
        Ok(_) => {
            let event = json!({
                "event": "product.created",
                "product_id": product.product_id,
                "sku": product.sku,
                "name": product.name,
                "price": product.price
            });

            let event_owned = event.to_string();
            let product_id = product.product_id.to_string();
            let record = FutureRecord::to("product.created")
                .payload(&event_owned)
                .key(&product_id);
            match state.producer.send(record, Duration::from_secs(0)).await {
                Ok(_) => HttpResponse::Ok().json(&product),
                Err((e, _)) => HttpResponse::InternalServerError().json(format!("Kafka error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("DB error: {}", e)),
    }
}

#[get("/products")]
async fn get_products(state: web::Data<AppState>) -> HttpResponse {
    let cursor = state.products.find(None, None).await;
    match cursor {
        Ok(mut cursor) => {
            let mut products = Vec::new();
            while cursor.advance().await.unwrap() {
                products.push(cursor.deserialize_current().unwrap());
            }
            HttpResponse::Ok().json(products)
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("DB error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());
    let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());

    let client = Client::with_uri_str(&mongodb_uri).await.unwrap();
    let db = client.database("ecommerce");
    let products = db.collection::<Product>("products");
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .create()
        .expect("Producer creation failed");

    let state = web::Data::new(AppState { products, producer });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(create_product)
            .service(get_products)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}