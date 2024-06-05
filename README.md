# Rust + Diesel + Sqlite example

To run example use `cargo run um`   
To build production executable`cargo build --release` 

P.S 
Diesel migrations will be applied by `build.rs` script

## Used Tools & Commands 
* Diesel CLI
    * `diesel migration generate create_posts`
    * `diesel migration generate fixture_posts`
    * `diesel print-schema --database-url "file:db.sqlite" > src/schema.rs`
* DBeaver 