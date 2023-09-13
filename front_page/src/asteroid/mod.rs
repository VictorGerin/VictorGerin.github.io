extern crate nalgebra as na;

mod data;
mod drawable;
mod entity;
mod game;
mod shader;

pub use data::*;
pub use drawable::*;
pub use entity::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use std::{cell::RefCell, rc::Rc};

use crate::asteroid::game::{ButtonState, MouseInput};

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

            let pos: Point2<f64> = Point2::new(
                ((event.offset_x() * 2) as f64 / game.canvas_dim.x) - 1f64,
                ((event.offset_y() * -2) as f64 / game.canvas_dim.y) + 1f64,
            );

            game.set_mouse_input(MouseInput {
                pos,
                left: event.buttons() == 1,
                right: event.buttons() == 2,
            });
        }
    };

    // let prevent_context = |event: MouseEvent| {
    //     event.prevent_default();
    // };

    // let k_down_event = {
    //     let game = game.clone();
    //     move |event: KeyboardEvent| {
    //         let mut game = game.borrow_mut();
    //         let game = game.as_mut().unwrap();

    //         if event.key() == " " {
    //             if let ButtonState::Pressed = game.input.keyboard.space {
    //                 game.input.keyboard.space = ButtonState::Hold;
    //             } else {
    //                 game.input.keyboard.space = ButtonState::Pressed;
    //             }
    //         }
    //     }
    // };

    // let k_up_event = {
    //     let game = game.clone();
    //     move |event: KeyboardEvent| {
    //         let mut game = game.borrow_mut();
    //         let game = game.as_mut().unwrap();

    //         if event.key() == " " {
    //             game.input.keyboard.space = ButtonState::Released;
    //         }
    //     }
    // };

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

    html! {
        <>
            <canvas
            onmousemove={m_event}
            style="border: 1px solid"
            tabindex="1"
            ref={canvas} width="400" height="400" />
        </>
    }
}
