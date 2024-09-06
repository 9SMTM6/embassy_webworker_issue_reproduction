use embassy_time::Timer;
use wasm_bindgen::prelude::*;

const SEC_WAIT: u64 = 1;

#[embassy_executor::task]
async fn ticker_main() {
    loop {
        log::info!("tick from main thread");

        Timer::after_secs(SEC_WAIT).await;
    }
}

#[embassy_executor::task]
async fn ticker_webworker() {
    loop {
        log::info!("tick from webworker");
        Timer::after_secs(SEC_WAIT).await;
        // wasm_thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[wasm_bindgen(start)]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
    executor.start(|spawner| {
        spawner.spawn(ticker_main()).unwrap();
    });
    wasm_thread::spawn(|| {
        log::info!("Hello from webworker");
        let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
        executor.start(|spawner| {
            spawner.spawn(ticker_webworker()).unwrap();
        });
    });
}
