use embassy_time::Timer;
use wasm_bindgen::prelude::*;

#[embassy_executor::task]
async fn ticker_main() {
    loop {
        log::info!("tick from main thread");

        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn ticker_webworker() {
    loop {
        log::info!("tick from webworker");

        Timer::after_secs(1).await;
    }
}

#[wasm_bindgen(start)]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
    // Don't ask me why they're are named differently. They do the same AFAICT.
    executor.start(|spawner| {
        spawner.spawn(ticker_main()).unwrap();
    });    
    wasm_thread::spawn(|| {
        log::info!("Hello from thread");
        let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
        // Don't ask me why they're are named differently. They do the same AFAICT.
        executor.start(|spawner| {
            spawner.spawn(ticker_webworker()).unwrap();
        });
    });
}
