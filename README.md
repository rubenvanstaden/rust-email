# Rust Email Newsletter

A personal project to understand the entire workflow of Rust as a backend.

```shell
# Spin up the database
./scripts/init_db.sh

# Make sure environment is clean
cargo clean

# Run the intergration tests
cargo test

# Run test with log and correlation ID highlighted
TEST_LOG=true cargo test | grep request_id

# Run server
cargo run

# In another terminal curl the endpoint
curl -i -X POST -d 'email=alice@gmail.com&name=Alice' http://127.0.0.1:8000/subscriptions
```
