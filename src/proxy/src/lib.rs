use std::cell::RefCell;

use candid::Principal;
pub use common::info;
use futures::future::join_all;
use ic_cdk::{api::call::CallResult, call, update};
#[update]
fn set_server(principal: String) {
    SERVER.with(|server| {
        *server.borrow_mut() = Some(principal);
    });
}

#[update]
async fn call_server_update_parallel(count: u32) -> Result<u64, Vec<String>> {
    let now = ic_cdk::api::time();
    let result = parallel_call(count, true).await;
    if let Err(err) = result {
        info(format!("Error: {:?}", err));
        return Err(err);
    }
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    Ok(elapsed)
}

#[update]
async fn call_server_query(count: u32) -> Result<u64, String> {
    let now = ic_cdk::api::time();
    query_call(count).await?;

    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    Ok(elapsed)
}

#[update]
async fn call_server_query_parallel(count: u32) -> Result<u64, Vec<String>> {
    let now = ic_cdk::api::time();
    let _ = parallel_call(count, false).await?;
    let elapsed_nanosec = ic_cdk::api::time() - now;
    let elapsed = elapsed_nanosec / 1_000_000;
    Ok(elapsed)
}

async fn parallel_call(count: u32, update: bool) -> Result<(), Vec<String>> {
    let results = if update {
        join_all((0..count).map(|_| update_call(1))).await
    } else {
        join_all((0..count).map(|_| query_call(1))).await
    };
    let errors: Vec<String> = results
        .iter()
        .filter_map(|r| match r {
            Err(err) => Some(err.clone()),
            _ => None,
        })
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
#[update]
async fn update(_: String) {
    let server = SERVER.with(|server| server.borrow().as_ref().map(|s| s.clone()));
    if server.is_none() {
        return;
    }
    let res = update_call(1).await;
    if let Err(err) = res {
        info(format!("Error: {:?}", err));
    }
}
async fn update_call(count: u32) -> Result<(), String> {
    for _ in 0..count {
        let res: CallResult<()> = call(server(), "update", ("test".to_string(),)).await;
        if let Err(err) = res {
            return Err(err.1);
        }
    }
    Ok(())
}
async fn query_call(count: u32) -> Result<(), String> {
    for _ in 0..count {
        let res: CallResult<()> = call(server(), "query", ("test".to_string(),)).await;
        if let Err(err) = res {
            return Err(err.1);
        }
    }
    Ok(())
}

fn server() -> Principal {
    SERVER.with(|server| server.borrow().as_ref().unwrap().parse().unwrap())
}

// states
thread_local! {
    static SERVER: RefCell<Option<String>> = RefCell::new(None);
}
ic_cdk::export_candid!();
