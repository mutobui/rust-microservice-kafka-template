# Rust Microservice Fafka Template
![Ecommerce](https://github.com/user-attachments/assets/4b53dafe-ea48-47bd-bd76-22e47c72ad40)

This project implements a set of microservices for an e-commerce platform using Rust, MongoDB, and Kafka. The services include:

- **Product Management**: Handles product creation and retrieval, publishing events to Kafka.
- **Order Management**: Manages orders, interacting with the inventory service.
- **Inventory Management**: Tracks product inventory levels.

## Prerequisites

- **Rust**: Version 1.82 or later (install via [rustup](https://rustup.rs/)).
- **Docker**: For running MongoDB and Kafka dependencies.
- **Cargo Watch** (optional): For hot reloading during development (`cargo install cargo-watch`).
- **curl** or **Postman**: For testing API endpoints.

## Project Structure

```
.
├── Cargo.toml              # Rust project configuration
├── services/               # Microservices source code
│   ├── product-management/
│   ├── order-management/
│   ├── inventory-management/
|   ├── models/ 
├── data/                   # Persistent data for MongoDB and Kafka
├── docker-compose.yml      # Development Docker configuration
```

## Setup for Local Development

### 1. Start Dependencies
Run MongoDB and Kafka using Docker:

```bash
docker-compose up -d
```

This starts:
- MongoDB on `localhost:27017`
- Kafka on `localhost:9092`

### 2. Test the API
Create a product:

```bash
curl -X POST http://localhost:8081/products \
  -H "Content-Type: application/json" \
  -d '{"sku":"PEN001","name":"Black Pen","price":1.99}'
```

List products:

```bash
curl http://localhost:8081/products
```

## 3. Create Kafka Topics
```bash
docker exec -it <kafka-container-id> kafka-topics --create --topic product.created --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1

docker exec -it <kafka-container-id> kafka-topics --create --topic cart.updated --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1

docker exec -it <kafka-container-id> kafka-topics --create --topic order.created --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1

docker exec -it <kafka-container-id> kafka-topics --create --topic payment.processed --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1

docker exec -it <kafka-container-id> kafka-topics --create --topic fulfillment.updated --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
```
