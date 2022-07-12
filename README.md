## build
```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
```

## run


```
docker run --name redis -d -p 6379:6379 redis redis-server --requirepass xxx

SESSION_AUTH="xxx" REDIS_URL="redis://xxx:19000/" ./redis-check
```