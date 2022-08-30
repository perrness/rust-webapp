use yew::{html, Properties, function_component, use_context};
use crate::app::Theme;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub link: String,
}

#[function_component(LinkComponent)]
pub fn link_component(props: &Props) -> Html {
    let theme = use_context::<Theme>().expect("No context found");

    html! {
        <div class={format!("link-container {}", theme.theme)}>
            <a href={ format!("{}", &props.link) }>{ &props.text }</a>
        </div>
    }
}
