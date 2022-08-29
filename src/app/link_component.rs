use yew::{Component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub link: String,
}

pub struct LinkComponent;

impl Component for LinkComponent {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        LinkComponent
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="link-container">
                <a href={ format!("{}", &ctx.props().link) }>{ &ctx.props().text }</a>
            </div>
        }
    }
}
