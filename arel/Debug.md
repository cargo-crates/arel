### Debug 

```bash
# in workspace
cargo run --bin arel -p arel --features=sqlite,tokio
cargo expand --bin arel -p arel --features=sqlite,tokio
cargo test --features=mysql,tokio
# only test sqlx about
cargo test sqlx --features=sqlite,tokio
```