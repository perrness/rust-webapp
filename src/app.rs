use yew::prelude::*;
use crate::app::link_component::LinkComponent;

mod link_component;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Per Næss" }</h1>
            <span class="subtitle">{ "Platform Engineeer" }</span>
            <LinkComponent link="https://github.com/perrness" text="Github"/>
            <LinkComponent link="https://linkedin.com/in/perness" text="LinkedIn"/>
        </main>
    }
}
