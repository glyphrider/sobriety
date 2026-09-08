#![cfg(target_arch = "wasm32")]

use std::time::Duration;

use sobriety::components::coin::{Coin, Props as CoinProps};
use sobriety::components::continuous_sobriety::{ContinuousSobriety, Props as DaysProps};
use wasm_bindgen_test::*;
use yew::platform::time::sleep;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn coin_renders_the_given_years_as_text() {
    let root = gloo::utils::document().create_element("div").unwrap();
    yew::Renderer::<Coin>::with_root_and_props(
        root.clone(),
        CoinProps { years: "XV".to_string() },
    )
    .render();
    sleep(Duration::ZERO).await;

    assert!(root.inner_html().contains("XV"));
}

#[wasm_bindgen_test]
async fn continuous_sobriety_renders_the_given_day_count() {
    let root = gloo::utils::document().create_element("div").unwrap();
    yew::Renderer::<ContinuousSobriety>::with_root_and_props(root.clone(), DaysProps { days: 42 })
        .render();
    sleep(Duration::ZERO).await;

    let html = root.inner_html();
    assert!(html.contains("42"));
    assert!(html.contains("days of continuous sobriety"));
}
