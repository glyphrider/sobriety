use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub days: i64,
}

#[function_component(ContinuousSobriety)]
pub fn render(props: &Props) -> Html {
    html! {
        <div class="days">
            { &props.days } { " days of continuous sobriety" }
        </div>
    }
}