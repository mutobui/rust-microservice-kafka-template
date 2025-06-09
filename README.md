# Rust Microservice Fafka Template
![Ecommerce](https://github.com/user-attachments/assets/4b53dafe-ea48-47bd-bd76-22e47c72ad40)

This project implements a set of microservices for an e-commerce platform using Rust, MongoDB, and Kafka. The services include:

Product Management: Handles product creation and retrieval, publishing events to Kafka.
Order Management: Manages orders, interacting with the inventory service.
Inventory Management: Tracks product inventory levels.

## Prerequisites

Rust: Version 1.82 or later (install via rustup).
Docker: For running MongoDB and Kafka dependencies.
Cargo Watch (optional): For hot reloading during development (cargo install cargo-watch).
curl or Postman: For testing API endpoints.

## Project Structure
.
├── Cargo.toml              # Rust project configuration
├── services/               # Microservices source code
│   ├── product-management/
│   ├── order-management/
│   ├── inventory-management/
├── models/                 # Shared data models
├── data/                   # Persistent data for MongoDB and Kafka
├── docker-compose.yml      # Production Docker configuration
├── docker-compose.dev.yml  # Local development Docker configuration
├── .env.*                  # Environment variables for each service

## Setup for Local Development
1. Start Dependencies
Run MongoDB and Kafka using Docker:
docker-compose up -d

This starts:

MongoDB on localhost:27017
Kafka on localhost:9092

2. Run Services Locally
Each service runs independently using cargo. Use separate terminals for each.
Product Management
export RUST_LOG=debug
cargo watch -x 'run --bin product-management' --env-file .env.product-management

Order Management
export RUST_LOG=debug
cargo watch -x 'run --bin order-management' --env-file .env.order-management

Inventory Management
export RUST_LOG=debug
cargo watch -x 'run --bin inventory-management' --env-file .env.inventory-management

Environment files (e.g., .env.product-management) are preconfigured with:

MONGODB_URI=mongodb://localhost:27017
KAFKA_BROKERS=localhost:9092
SERVICE_NAME=<service-name>

3. Test the API
Create a product:
curl -X POST http://localhost:8081/products \
  -H "Content-Type: application/json" \
  -d '{"sku":"PEN001","name":"Black Pen","price":1.99}'

List products:
curl http://localhost:8081/products

## Create Kafka Topics
docker exec -it <kafka-container-id> kafka-topics --create --topic product.created --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic cart.updated --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic order.created --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic payment.processed --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic fulfillment.updated --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
