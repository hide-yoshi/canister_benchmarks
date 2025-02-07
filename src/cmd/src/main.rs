use std::{fs::OpenOptions, time::Instant};

const CANISTER_COUNT: u32 = 10;
const CALL_COUNT: u32 = 20;
#[tokio::main]
async fn main() {
    call_script(CALL_COUNT).await;
}
async fn call_script(call_count: u32) {
    let canister_name = "proxy1".to_string();
    let r = tokio::process::Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name.clone())
        .arg("call_server_update_parallel")
        .arg(format!("{}", call_count))
        .arg("--ic")
        .arg("--output")
        .arg("json")
        .output()
        .await
        .unwrap();
    let log = tokio::process::Command::new("dfx")
        .arg("canister")
        .arg("logs")
        .arg(canister_name.clone())
        .arg("--ic")
        .output()
        .await
        .unwrap();
    println!("log: {}", String::from_utf8_lossy(&log.stdout));
}
