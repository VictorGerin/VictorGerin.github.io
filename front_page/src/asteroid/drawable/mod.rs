extern crate nalgebra as na;

mod object;

use na::{Point2, Vector2};
pub use object::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub trait Drawable {
    fn draw(
        &self,
        context: &WebGlRenderingContext,
        offset: Point2<f64>,
        rotation: f64,
    ) -> Result<(), JsValue>;
}
