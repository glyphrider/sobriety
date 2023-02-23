use yew::prelude::*;
use chrono::{ Local, NaiveDate, NaiveDateTime };

mod components;

use components::continuous_sobriety::ContinuousSobriety;
use components::coin::Coin;

fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

#[function_component(App)]
pub fn render() -> Html {
    // put the current time into state, and get back a UseStateHandle
    let now = use_state(|| get_now());

    // here are all the calculations that ultimately product *roman* and *days*
    let sober = NaiveDate::from_ymd_opt(2009,6,20).unwrap();
    let formatted_date = sober.format("%A, %B %e, %Y");
    let years = now.date().years_since(sober).unwrap();
    let roman = roman::convert::to(years as u16);
    let days = now.date().signed_duration_since(sober).num_days();

    // create a clone of the UseStateHandle, so rust will allow it to be moved into the closure that follows
    // this is important because the closure will have a very different lifespan than fn App().
    let cloned_now = now.clone();

    // create an interval timer (every 5 seconds) that executes this closure
    // again, cloned_now is *moved* into the closure to avoid lifespan issues
    // somehow, both handles allows us to manipulate and access the same state (magic!)
    gloo::timers::callback::Interval::new(5000, move || {
        cloned_now.set(get_now());
    }).forget();

    // this is the return: pseudo-html just like React
    // 
    html! {
        <div class="sobriety">
            <h1>{ "My sobriety timeline" }</h1>
            <p>{ formatted_date }<i>{ " through today" }</i></p>
            <Coin years={ roman } />
            <ContinuousSobriety days={ days } />
        </div>
    }
}
