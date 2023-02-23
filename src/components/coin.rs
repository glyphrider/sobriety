use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub years: String,
}

#[function_component(Coin)]
pub fn render(props: &Props) -> Html {

    html! {
        <div class="years">
        { &props.years }
        </div>
    }
}