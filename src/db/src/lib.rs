use std::cell::RefCell;

use candid::Principal;
pub use common::info;
use ic_cdk::{call, query, update};
#[query]
fn greet(name: String) -> String {
    info("greet".to_string());
    format!("Hello, {}!", name)
}

#[update]
fn set_server(principal: String) {
    SERVER.with(|server| {
        *server.borrow_mut() = Some(principal);
    });
}

#[update]
async fn call_server_update(count: u32) -> u64 {
    let now = ic_cdk::api::time();
    update_call(count).await;
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    elapsed
}

#[update]
async fn call_server_update_parallel(count: u32) -> u64 {
    let now = ic_cdk::api::time();
    parallel_call(count, true).await;
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    elapsed
}

#[update]
async fn call_server_query(count: u32) -> u64 {
    let now = ic_cdk::api::time();
    query_call(count).await;
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    elapsed
}

#[update]
async fn call_server_query_parallel(count: u32) -> u64 {
    let now = ic_cdk::api::time();
    parallel_call(count, false).await;
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    elapsed
}

async fn parallel_call(count: u32, update: bool) {
    if update {
        futures::future::join_all((0..count).map(|_| update_call(1))).await;
    } else {
        futures::future::join_all((0..count).map(|_| query_call(1))).await;
    }
}

async fn update_call(count: u32) {
    let now = ic_cdk::api::time();
    for _ in 0..count {
        let () = call(server(), "update", ("test".to_string(),))
            .await
            .unwrap();
    }
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    info(format!("Elapsed time: {}ms", elapsed));
}
async fn query_call(count: u32) {
    let now = ic_cdk::api::time();
    for _ in 0..count {
        let () = call(server(), "query", ("test".to_string(),))
            .await
            .unwrap();
    }
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    info(format!("Elapsed time: {}ms", elapsed));
}

fn server() -> Principal {
    SERVER.with(|server| server.borrow().as_ref().unwrap().parse().unwrap())
}

// states
thread_local! {
    static SERVER: RefCell<Option<String>> = RefCell::new(None);
}
ic_cdk::export_candid!();
