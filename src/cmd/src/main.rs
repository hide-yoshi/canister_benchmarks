use std::{fs::OpenOptions, time::Instant};

const CANISTER_COUNT: u32 = 10;
const CALL_COUNT: u32 = 20;
#[tokio::main]
async fn main() {
    call_script(CANISTER_COUNT, CALL_COUNT).await;
}
async fn call_script(count: u32, call_count: u32) {
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

async fn write_result_to_csv(start: Instant, end: Instant, call_count: u32, canister_name: String) {
    let elapsed = end - start;
    let elapsed = elapsed.as_millis();
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("result_parallel_10_canisters_same_subnet.csv")
        .unwrap();
    let mut wtr = csv::Writer::from_writer(f);
    wtr.write_record(&[
        format!("{}", call_count),
        format!("{}", elapsed),
        canister_name,
    ])
    .unwrap();
    wtr.flush().unwrap();
}
