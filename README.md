# AI Summarize API

This project aims to learn rust by creating a REST API that contains ai summarization features. You can register and login to get an access token, and then do summarization using the access you get.

Currently im using Axtic-web for web service. It's more simple and support for api documentation using utoipa.

## Module
- `actix-web` for asynchronous runtime 
- `sea-orm` for ORM 
- `sea-orm-cli` for migration database and generation migration script
- `utoipa` for api documentation generator (openapi)
- `serde` and `serde-json` for serializing and deserializing struct and json stuct
- `pdf-extract` for extraction pdf file content or text
- `docx-rust` for extraction docx file content or text 
- any utility module, like `jsonwebtoken`, `bcrypt`, `reqwest`, etc

## Migration

Before doing migration, make sure you have installed sea-orm-cli

```
cargo install sea-orm-cli@1.1.0
```

Here the step for migration:
- run `sea-orm-cli migrate generate <migration-name>`
- modify in file `migration/src/m[datetime]-[migration-name].rs`
- run `sea-orm-cli migrate up`
- if you want to rollback, run `sea-orm-cli migrate down`

## Run application

Make sure to create file `.env` for environment requirement. Please check the example environment `.env.example`

Then, run `cargo run`

## API Documentation

After running the application, you can check documentation in path `<localhost>/swagger-ui/index.html`

## Update

`19-03-2025`
  - change from using `sea-orm` to `sqlx`, because more native and lightweight
  - simplified apidocs swagger to `docs.rs`