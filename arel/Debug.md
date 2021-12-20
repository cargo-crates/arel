### Debug 

```bash
# in workspace
cargo expand -p example_sqlite
cargo test -p arel --features=mysql
# only test sqlx about
cargo test sqlx -p arel --features=sqlite
```

bugs

`
字段名 r#type
`