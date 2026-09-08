// this file contains the root (App) component
// the remaining components are housed in the components directory

use yew::prelude::*;
use chrono::{ Local, NaiveDate, NaiveDateTime };

pub mod components;

use components::continuous_sobriety::ContinuousSobriety;
use components::coin::Coin;

fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

fn sober_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2009, 6, 20).unwrap()
}

fn roman_years_since(now: NaiveDateTime, sober: NaiveDate) -> String {
    let years = now.date().years_since(sober).unwrap();
    roman::convert::to(years as u16)
}

fn days_since(now: NaiveDateTime, sober: NaiveDate) -> i64 {
    now.date().signed_duration_since(sober).num_days()
}

#[function_component(App)]
pub fn render() -> Html {
    // put the current time into state, and get back a UseStateHandle
    let now = use_state(|| get_now());

    // here are all the calculations that ultimately product *roman* and *days*
    let sober = sober_date();
    let formatted_date = sober.format("%A, %B %e, %Y");
    let roman = roman_years_since(*now, sober);
    let days = days_since(*now, sober);

    // create a clone of the UseStateHandle, so rust will allow it to be moved into the closure that follows
    // this is important because the closure will have a very different lifespan than fn App().
    let cloned_now = now.clone();

    // create an interval timer (every 5 seconds) that executes this closure
    // again, cloned_now is *moved* into the closure to avoid lifespan issues
    // somehow, both handles allows us to manipulate and access the same state (magic!)
    // use_effect_with with a `()` dependency runs this setup exactly once on mount,
    // rather than on every render (which would leak a new Interval each time `now` changes)
    use_effect_with_deps(
        move |_| {
            let interval = gloo::timers::callback::Interval::new(5000, move || {
                cloned_now.set(get_now());
            });
            move || drop(interval)
        },
        (),
    );

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    fn at(date: NaiveDate) -> NaiveDateTime {
        NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }

    #[test]
    fn roman_years_since_on_anniversary() {
        let sober = sober_date();
        let now = at(NaiveDate::from_ymd_opt(2024, 6, 20).unwrap());
        assert_eq!(roman_years_since(now, sober), "XV");
    }

    #[test]
    fn roman_years_since_rounds_down_before_anniversary() {
        let sober = sober_date();
        let now = at(NaiveDate::from_ymd_opt(2024, 6, 19).unwrap());
        assert_eq!(roman_years_since(now, sober), "XIV");
    }

    #[test]
    fn days_since_same_day_is_zero() {
        let sober = sober_date();
        assert_eq!(days_since(at(sober), sober), 0);
    }

    #[test]
    fn days_since_counts_elapsed_full_days() {
        let sober = sober_date();
        let now = at(sober + chrono::Duration::days(30));
        assert_eq!(days_since(now, sober), 30);
    }
}
