use crate::asteroid::Asteroid;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    log::info!("INIT !! 2");
    let texto = use_state(|| "".to_owned());

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let temp = Request::get("https://muddy-leaf-3674.fly.dev/teste")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                log::info!("temp : '{}'", temp);
            });
            || ()
        },
        (),
    );

    html! {
        <main>
            <Asteroid />
            // <h1>{ "E ai Davi blz ?" }</h1>
            // <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
