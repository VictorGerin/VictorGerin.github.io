use crate::asteroid::Asteroid;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    log::info!("INIT !! 2 !");
    let texto: UseStateHandle<String> = use_state(|| "".to_owned());

    {
        let texto = texto.clone();
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

                    texto.set(temp);
                });
                || ()
            },
            (),
        );
    }

    let texto2 = texto.clone();

    html! {
        <main>
            <Asteroid />
            if (*texto).len() > 0 {
                <h1>{ (*texto2).to_string() }</h1>
            }
        </main>
    }
}
