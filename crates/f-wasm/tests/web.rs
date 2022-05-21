mod common;

use common::timeout::Timeout;
use std::time::Duration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use f_wasm::app::App as YewApp;
use yew::start_app;

#[wasm_bindgen_test]
async fn home_page_has_podcasts() {
    start_app::<YewApp>();

    let podcasts = gloo_utils::document().get_elements_by_class_name("podcast-preview");

    console_log!("Initial podcasts length: {}", podcasts.length());
    assert_eq!(podcasts.length(), 1);

    console_log!("Waiting for podcasts to load.");
    Timeout::new(Duration::new(10, 0)).await;

    // console_log!("Loaded podcasts length: {}", podcasts.length());
    // assert_eq!(podcasts.length(), 10);
}
