### Debug 

```bash
# in workspace
cargo expand --bin arel -p arel --features=sqlite,tokio
cargo test -p arel --features=mysql
# only test sqlx about
cargo test -p arel --features=sqlite
```