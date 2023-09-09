use gloo_net::http::Request;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{prelude::*, platform::pinned::oneshot};

async fn await_some_time(time: i32) {

    let  (sender, reciver) = oneshot::channel::<i32>();


    let call = Closure::once(|| {
        sender.send(0).unwrap();
    });

    let window = web_sys::window().expect("no global `window` exists");
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(call.as_ref().unchecked_ref(), time);
    let _ = reciver.await;
}

#[function_component]
pub fn example() -> Html {
    
    let texto: UseStateHandle<String> = use_state(|| "".to_owned());

    {
        let texto = texto.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {

                    await_some_time(2000).await;

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