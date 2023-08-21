use crate::asteroid::Asteroid;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {

    html! {
        <main>
            <Asteroid />
            // <h1>{ "E ai Davi blz ?" }</h1>
            // <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
