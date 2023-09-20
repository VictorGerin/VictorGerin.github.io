mod app;
mod asteroid;
mod hook;

use std::collections::HashSet;

use app::App;
use nalgebra::Matrix2xX;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Object {
    hit_box: Matrix2xX<f64>,
    hit_box_edge: Vec<usize>,
    hit_box_obj: Vec<usize>,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();

    // let a = &[&[1, 2, 3][..], &[4, 5, 6][..]][..];

    // let a: Vec<&i32> = a
    //     .iter()
    //     .flat_map(|x| {
    //         let temp = *x;
    //         temp
    //     })
    //     .collect();
    // println!("{:?}", a);
}
