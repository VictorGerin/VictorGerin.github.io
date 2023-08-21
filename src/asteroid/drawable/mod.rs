extern crate nalgebra as na;

mod drawable_collection;
mod drawable_type;
mod object;

pub use drawable_collection::*;
pub use drawable_type::*;
use na::{Vector2, Point2};
pub use object::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub trait Drawable {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        offset: Point2<f64>,
        rotation: f64,
        scale: f64,
    ) -> Result<(), JsValue>;
}
