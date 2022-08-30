use std::ops::Deref;

use yew::prelude::*;
use crate::app::link_component::LinkComponent;

mod link_component;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Theme {
    theme: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let theme_state = use_state(|| Theme {
        theme: "light".to_owned(),
    });
    let onclick = {
        let theme_state = theme_state.clone();
        Callback::from(move |_event: MouseEvent| {
            let mut theme = theme_state.deref().clone();
            
            theme.theme = match theme_state.theme.as_str() {
                "light" => "dark".to_owned(),
                "dark" => "light".to_owned(),
                _ => "light".to_owned(),
            };

            theme_state.set(theme);
        })
    };

    html! {
        <ContextProvider<Theme> context={(*theme_state).clone()}>
            <main class={format!("{}", theme_state.theme)}>
                <h1>{ "Per Næss" }</h1>
                <span class="subtitle">{ "Platform Engineeer" }</span>
                <LinkComponent link="https://github.com/perrness" text="Github"/>
                <LinkComponent link="https://linkedin.com/in/perness" text="LinkedIn"/>
                <button {onclick}>{"💡"}</button>
            </main>
        </ContextProvider<Theme>>
    }
}
