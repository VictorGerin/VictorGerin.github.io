use crate::asteroid::Asteroid;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <body>
                // <header>
                //     <span class="name">{"Victor Lacerda"}</span>
                //     <span class="email">{"gerinlacerda@gmail.com"}</span>
                // </header>
                <main>
                    // <div>
                    //     <h1>{"Teste"}</h1>
                    // </div>
                    <Asteroid />
                </main>
            </body>
        </>
    }
}
