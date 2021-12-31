# Manger
A **Telegram bot** that will keep you updated with the latest updates to your **favorite manga**!

### Dependencies
* Rust
* NodeJS
* Docker
* Telegram API (*teloxide*)

### Local development
1. Create `.env` file:
```
ADMIN_ID=
TELOXIDE_TOKEN=
BOT_NAME=dev
DATABASE_URL=
```
2. Run `main.rs`

### Migrations
1.Run migrations:
```bash
make migrate-up
```
2. Redo migrations:
```bash
make migrate-down
```
3. Create a new migration:
```bash
make migrate-create {name}
```

### Release
Just run `make release`