use anyhow::Result;
use redis::Value;
use std::env;
use std::time::{Duration, Instant};
use tracing::{info, instrument};
// use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let addr = &env::var("REDIS_URL")?;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return run_interactive(addr).await;
    }

    let key = &args[1];
    m_get(addr, key).await?;

    Ok(())
}

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
async fn run_interactive(addr: &str) -> Result<()> {
    let mut stdout = io::stderr();
    let stdin = io::stdin();
    let msg = "请按格式输入[命令] [参数..]\n";
    let unsupported = "暂不支持MGET与GET以外的命令";
    stdout.write_all(msg.as_bytes()).await?;
    stdout.write_all(b">> ").await?;

    let client = redis::Client::open(addr)?;
    let mut con = client.get_async_connection().await?;
    if let Ok(x) = env::var("SESSION_AUTH") {
        redis::cmd("AUTH").arg(x).query_async(&mut con).await?;
    }

    let mut lines = BufReader::new(stdin).lines();
    while let Some(line) = lines.next_line().await? {
        if line == "q" || line == "quit" {
            return Ok(());
        }

        let command_and_args: Vec<&str> = line.split(' ').collect();
        let command = command_and_args[0].to_uppercase();

        if command != "GET" && command != "MGET" {
            stdout.write_all(unsupported.as_bytes()).await?;
            stdout.write_all(msg.as_bytes()).await?;
            stdout.write_all(b">> ").await?;
            continue;
        }

        // let mut con = client.get_async_connection().await?;
        // if let Ok(x) = env::var("SESSION_AUTH") {
        //     redis::cmd("AUTH").arg(x).query_async(&mut con).await?;
        // }

        let start = Instant::now();
        let result = if command_and_args.len() > 1 {
            let args = &command_and_args[1..];
            redis::cmd(&command).arg(args).query_async(&mut con).await?
        } else {
            redis::cmd(&command).query_async(&mut con).await?
        };
        let duration = start.elapsed();

        stdout
            .write_all(output(duration, result).as_bytes())
            .await?;
        stdout.write_all(msg.as_bytes()).await?;
        stdout.write_all(b">> ").await?;
    }
    Ok(())
}

#[instrument(level = "info", name = "m_get")]
async fn m_get(addr: &str, key: &str) -> Result<()> {
    tracing_subscriber::fmt()
        // .with_span_events(FmtSpan::ACTIVE)
        .init();

    let client = redis::Client::open(addr)?;
    let mut con = client.get_async_connection().await?;

    if let Ok(x) = env::var("SESSION_AUTH") {
        redis::cmd("AUTH").arg(x).query_async(&mut con).await?;
    }
    let start = Instant::now();
    // let result = redis::cmd("MGET").arg(keys).query_async(&mut con).await?;
    let result = redis::cmd("GET").arg(key).query_async(&mut con).await?;
    let duration = start.elapsed();

    info!("{}", output(duration, result));

    Ok(())
}

fn output(duration: Duration, result: redis::Value) -> String {
    let result = match result {
        Value::Int(r) => format!("结果: {r}"),
        Value::Data(r) => format!("结果: {}", String::from_utf8_lossy(&r)),
        Value::Bulk(rs) => format!("结果: {rs:?}"),
        Value::Status(s) => format!("结果: {s}"),
        Value::Okay => "结果: ok".to_string(),
        Value::Nil => "结果: nil".to_string(),
    };
    format!("\n执行时间: {duration:?}  {result}\n")
}
