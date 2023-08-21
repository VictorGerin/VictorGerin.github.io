use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use yew::{hook, use_effect};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>) {
    window()
        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[hook]
pub fn use_framerate<F>(callback: F)
where
    F: FnMut(f64) + 'static,
{
    let mut callback = callback;
    use_effect(move || {
        //indica se deve parar a animação
        let stop = Rc::new(RefCell::new(false));
        //Referencia para a função de animação
        let f: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>> = Rc::new(RefCell::new(None));

        *f.borrow_mut() = {
            //clona as variáveis para serem usadas dentro da closure
            let f = f.clone();
            let stop = stop.clone();
            Some(Closure::new(move |time: JsValue| {
                callback(time.as_f64().unwrap());
                //Valida se a animação deve continuar
                if *stop.borrow() == false {
                    //programa a prox aniamção
                    request_animation_frame(&f);
                }
            }))
        };

        // //inicia a animação
        request_animation_frame(&f);

        //retorna uma closure para parar a animação
        move || {
            stop.replace(true);
        }
    });
}
