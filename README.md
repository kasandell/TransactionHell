#
```
docker-compose up postgres migrations
cargo build
```
if you're going to edit the db port etc, just make sure to change it in `db.rs` & `.env`