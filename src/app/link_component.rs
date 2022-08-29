use yew::{Component, html};

pub struct LinkComponent;

impl Component for LinkComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        LinkComponent
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="link-container">{ "Link to stuff here" }</div>
        }
    }
}
