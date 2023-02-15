use yew::prelude::*;
use chrono::{ Local, NaiveDate, NaiveDateTime };

fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

#[function_component]
fn App() -> Html {
    let sober = NaiveDate::from_ymd_opt(2009,6,20).unwrap();
    let now = use_state(|| get_now());
    let formatted_date = sober.format("%A, %B %e, %Y");
    let years = now.date().years_since(sober).unwrap();
    let roman = roman::convert::to(years as u16);
    let days = now.date().signed_duration_since(sober).num_days();

    let cloned_now = now.clone();
    gloo::timers::callback::Interval::new(5000, move || {
        cloned_now.set(get_now());
    }).forget();

    html! {
        <div class="sobriety">
            <h1>{ "My sobriety timeline" }</h1>
            <p>{ formatted_date }<i>{ " through today" }</i></p>
            <div class="years">
                { roman }
            </div>
            <div class="days">
                { days } { " days of continuous sobriety" }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
