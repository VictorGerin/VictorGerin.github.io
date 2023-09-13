extern crate nalgebra as na;
use super::drawable::*;

use na::{Point2, Vector2};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

#[derive(Debug, Clone)]
pub struct Entity {
    pub object: Object,
    pub pos: Point2<f64>,
    pub speed: Vector2<f64>,
    pub acc: Vector2<f64>,
    pub rotation: f64,
    pub delete_on_out_of_bounds: bool,
    pub max_speed_sqr: f64,
}
#[derive(Debug, Clone)]
pub struct EntityDrawable {
    pub object: ObjectDrawable,
    pub pos: Point2<f64>,
    pub speed: Vector2<f64>,
    pub acc: Vector2<f64>,
    pub rotation: f64,
    pub delete_on_out_of_bounds: bool,
    pub max_speed_sqr: f64,
}

impl TryFrom<&str> for Entity {
    type Error = serde_json::Error;
    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let ship: Object = data.try_into()?;
        Ok(Entity {
            object: ship,
            pos: Point2::default(),
            speed: Vector2::default(),
            rotation: Default::default(),
            acc: Vector2::default(),
            delete_on_out_of_bounds: true,
            max_speed_sqr: Default::default(),
        })
    }
}

impl Entity {
    pub fn load_gl(self, gl: &WebGlRenderingContext) -> EntityDrawable {
        let obj = self.object.load_gl(gl);
        EntityDrawable {
            object: obj,
            pos: self.pos,
            speed: self.speed,
            rotation: self.rotation,
            acc: self.acc,
            delete_on_out_of_bounds: self.delete_on_out_of_bounds,
            max_speed_sqr: self.max_speed_sqr,
        }
    }
}

impl EntityDrawable {
    pub fn update_physics(&mut self, delta: f64) {
        self.speed += self.acc * delta;
        if self.max_speed_sqr != 0.0 && self.speed.norm_squared() > self.max_speed_sqr {
            self.speed = self.speed.normalize() * self.max_speed_sqr.sqrt();
        }
        self.pos += self.speed * delta;
    }

    pub fn get_pos_center(&self) -> Point2<f64> {
        self.pos + self.object.dimentions() / 2.0
    }

    pub fn draw(&self, context: &WebGlRenderingContext) -> Result<(), JsValue> {
        self.draw_position(context, self.pos)
    }

    pub fn draw_position(
        &self,
        context: &WebGlRenderingContext,
        pos: Point2<f64>,
    ) -> Result<(), JsValue> {
        self.object.draw(context, pos, self.rotation, 0.0)
    }
}
