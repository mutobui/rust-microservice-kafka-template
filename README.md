docker exec -it <kafka-container-id> kafka-topics --create --topic product.created --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic cart.updated --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic order.created --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic payment.processed --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1
docker exec -it <kafka-container-id> kafka-topics --create --topic fulfillment.updated --bootstrap-server kafka:9092 --partitions 1 --replication-factor 1# rust-microservice-kafka-template
