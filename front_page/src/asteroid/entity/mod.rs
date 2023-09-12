extern crate nalgebra as na;
use super::drawable::*;

use na::{Point2, Vector2};
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext};

#[derive(Debug, Clone)]
pub struct Entity {
    object: Object,
    pos: Point2<f64>,
    speed: Vector2<f64>,
    acc: Vector2<f64>,
    rotation: f64,
    delete_on_out_of_bounds: bool,
    max_speed_sqr: f64,
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
    pub fn update_physics(&mut self, delta: f64) {
        self.speed += self.acc * delta;
        if self.max_speed_sqr != 0.0 && self.speed.norm_squared() > self.max_speed_sqr {
            self.speed = self.speed.normalize() * self.max_speed_sqr.sqrt();
        }
        self.pos += self.speed * delta;
    }

    pub fn get_object(&self) -> &Object {
        &self.object
    }

    pub fn get_pos(&self) -> Point2<f64> {
        self.pos
    }

    pub fn set_max_speed(&mut self, speed: f64) {
        self.max_speed_sqr = speed * speed;
    }

    pub fn get_pos_center(&self) -> Point2<f64> {
        self.get_pos() + self.get_object().dimentions() / 2.0
    }

    pub fn set_speed(&mut self, speed: Vector2<f64>) {
        self.speed = speed;
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
    }

    pub fn get_speed(&self) -> Vector2<f64> {
        self.speed
    }

    // pub fn get_rotation(&self) -> f64 {
    //     self.rotation
    // }

    pub fn get_acc(&self) -> Vector2<f64> {
        self.acc
    }

    pub fn set_acc(&mut self, acc: Vector2<f64>) {
        self.acc = acc;
    }

    pub fn draw(
        &self,
        context: &WebGlRenderingContext,
        gl_prg: &WebGlProgram,
    ) -> Result<(), JsValue> {
        self.draw_position(context, gl_prg, self.get_pos())
    }

    pub fn draw_position(
        &self,
        context: &WebGlRenderingContext,
        gl_prg: &WebGlProgram,
        pos: Point2<f64>,
    ) -> Result<(), JsValue> {
        self.object.draw(context, gl_prg, pos, self.rotation, 0.0)
    }

    pub fn set_pos(&mut self, new: Point2<f64>) {
        self.pos = new;
    }

    pub fn set_delete_on_out_of_bounds(&mut self, delete: bool) {
        self.delete_on_out_of_bounds = delete;
    }

    pub fn get_delete_on_out_of_bounds(&self) -> bool {
        self.delete_on_out_of_bounds
    }
}
