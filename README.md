## build
```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
```

## run


```
docker run --name redis -d -p 6379:6379 redis redis-server --requirepass e2H98GokB7weF0o3dg

SESSION_AUTH="e2H98GokB7weF0o3dg" REDIS_URL="redis://10.111.205.108:19000/" ./redis-check ugc/knowledgelevel.knowledge_score.uid:3033330525:season:1656604800
```