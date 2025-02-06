use ic_cdk::{query, update};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

thread_local! {
    static CALL_COUNT: std::sync::Mutex<u32> = std::sync::Mutex::new(0);
}

#[update]
async fn update(_: String) {
    CALL_COUNT.with(|count| {
        let mut count = count.lock().unwrap();
        *count += 1;
    });
    _fib(30);
}

fn _fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        _fib(n - 1) + _fib(n - 2)
    }
}

#[query]
async fn query(_: String) {
    CALL_COUNT.with(|count| {
        let count = count.lock().unwrap();
        println!("Call count: {}", *count);
    });
    _fib(30);
}

ic_cdk::export_candid!();

#[cfg(test)]
mod test {
    use crate::_fib;

    #[test]
    fn test_fib() {
        _fib(0);
    }
}
