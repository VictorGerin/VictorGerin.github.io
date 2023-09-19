extern crate nalgebra as na;

mod data;
mod drawable;
mod entity;
mod game;
mod shader;

pub use data::*;
pub use drawable::*;
pub use entity::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use std::{cell::RefCell, rc::Rc};

use crate::asteroid::game::ButtonState;

use super::hook::*;
use game::Game;
use na::Point2;
use web_sys::*;
use yew::prelude::*;

#[function_component]
pub fn Asteroid() -> Html {
    let game: Rc<RefCell<Option<Game>>> = Rc::new(RefCell::new(None));

    let canvas = use_node_ref();

    use_effect({
        let game = game.clone();
        let canvas = canvas.clone();
        move || {
            let canvas: HtmlCanvasElement = canvas.cast::<HtmlCanvasElement>().unwrap().clone();
            *game.borrow_mut() = Some(Game::new(canvas))
        }
    });

    //Função de animação chamada a cada frame do navegador
    use_framerate({
        let last_time = Rc::new(RefCell::new(0f64));
        let game = game.clone();

        move |time: f64| {
            let mut game = game.borrow_mut();
            let game = game.as_mut().unwrap();

            let delta = {
                let mut last_time = last_time.borrow_mut();
                let delta: f64 = time - *last_time;
                *last_time = time;
                delta
            };

            //Game loop
            game.game_loop(time, delta);
        }
    });

    let m_event = {
        let game = game.clone();
        move |event: MouseEvent| {
            let mut game = game.borrow_mut();
            let game = game.as_mut().unwrap();

            game.input.mouse.pos = Point2::new(event.offset_x() as f64, event.offset_y() as f64);
            game.update_input();
        }
    };

    let m_down_event = {
        let game = game.clone();
        move |event: MouseEvent| {
            let mut game = game.borrow_mut();
            let game = game.as_mut().unwrap();

            game.input.mouse.pos = Point2::new(event.offset_x() as f64, event.offset_y() as f64);
            match event.button() {
                0 => game.input.mouse.left = true,
                2 => game.input.mouse.right = true,
                _ => {}
            }
            game.update_input();
        }
    };

    let m_up_event = {
        let game = game.clone();
        move |event: MouseEvent| {
            let mut game = game.borrow_mut();
            let game = game.as_mut().unwrap();

            game.input.mouse.pos = Point2::new(event.offset_x() as f64, event.offset_y() as f64);
            match event.button() {
                0 => game.input.mouse.left = false,
                2 => game.input.mouse.right = false,
                _ => {}
            }
            game.update_input();
        }
    };

    let prevent_context = |event: MouseEvent| {
        event.prevent_default();
    };

    let k_down_event = {
        let game = game.clone();
        move |event: KeyboardEvent| {
            let mut game = game.borrow_mut();
            let game = game.as_mut().unwrap();

            if event.key() == " " {
                if let ButtonState::Pressed = game.input.keyboard.space {
                    game.input.keyboard.space = ButtonState::Hold;
                } else {
                    game.input.keyboard.space = ButtonState::Pressed;
                }
            }
        }
    };

    let k_up_event = {
        let game = game.clone();
        move |event: KeyboardEvent| {
            let mut game = game.borrow_mut();
            let game = game.as_mut().unwrap();

            if event.key() == " " {
                game.input.keyboard.space = ButtonState::Released;
            }
        }
    };

    // let k_hold_event = {
    //     let game = game.clone();
    //     move |event: KeyboardEvent| {
    //         let mut game = game.borrow_mut();
    //         let game = game.as_mut().unwrap();
    //         if event.key() == " " {
    //             game.input.keyboard.space = ButtonState::Hold;
    //         }
    //     }
    // };

    use_effect({
        let k_down_event: Closure<dyn Fn(KeyboardEvent)> = Closure::wrap(Box::new(k_down_event));
        move || {
            window()
                .unwrap()
                .add_event_listener_with_callback("keydown", k_down_event.as_ref().unchecked_ref())
                .unwrap();

            move || {
                window()
                    .unwrap()
                    .remove_event_listener_with_callback(
                        "keydown",
                        k_down_event.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        }
    });

    use_effect({
        let k_up_event: Closure<dyn Fn(KeyboardEvent)> = Closure::wrap(Box::new(k_up_event));
        move || {
            window()
                .unwrap()
                .add_event_listener_with_callback("keyup", k_up_event.as_ref().unchecked_ref())
                .unwrap();

            move || {
                window()
                    .unwrap()
                    .remove_event_listener_with_callback(
                        "keyup",
                        k_up_event.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        }
    });

    html! {
        <>
            <canvas
            oncontextmenu={prevent_context}
            onmousedown={m_down_event.clone()}
            onmouseup={m_up_event.clone()}
            onmousemove={m_event.clone()}
            style="border: 1px solid"
            ref={canvas} width="600" height="600" />
        </>
    }
}
