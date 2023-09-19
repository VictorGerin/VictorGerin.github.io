extern crate nalgebra as na;
use super::drawable::*;

use na::{Matrix2x3, Point2, Rotation2, Vector2, Vector3};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

#[derive(Debug, Clone)]
pub struct EntityDrawable {
    pub object: ObjectDrawable,
    pub pos: Point2<f64>,
    pub speed: Vector2<f64>,
    pub acc: Vector2<f64>,
    pub color: Vector3<f64>,
    pub rotation: f64,
    pub delete_on_out_of_bounds: bool,
    pub max_speed_sqr: f64,
}

impl EntityDrawable {
    pub fn load_gl(gl: &WebGlRenderingContext, json: &str) -> EntityDrawable {
        let entity: Object = json.try_into().unwrap();

        EntityDrawable {
            object: entity.load_gl(gl),
            pos: Point2::default(),
            speed: Vector2::default(),
            rotation: Default::default(),
            acc: Vector2::default(),
            delete_on_out_of_bounds: true,
            max_speed_sqr: Default::default(),
            color: Vector3::new(1.0, 0.0, 1.0),
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
        self.object.draw(context, pos, self.rotation, self.color)
    }

    fn get_min_max_from_proj(points: Matrix2x3<f64>, axis_proj: Vector2<f64>) -> (f64, f64) {
        let mut min_r1 = f64::INFINITY;
        let mut max_r1 = -f64::INFINITY;

        for p in points.column_iter() {
            let q = p.dotc(&axis_proj);

            min_r1 = q.min(min_r1);
            max_r1 = q.max(max_r1);
        }
        return (min_r1, max_r1);
    }

    fn triagle_hit(t1: Matrix2x3<f64>, t2: Matrix2x3<f64>) -> bool {
        let ncols = t1.ncols();
        for a in 0..ncols {
            let b = (a + 1) % ncols;
            let axis_proj: Vector2<f64> = t1.column(b) - t1.column(a);
            let axis_proj: Vector2<f64> = Vector2::new(-axis_proj.y, axis_proj.x).normalize();

            let (min_r1, max_r1) = EntityDrawable::get_min_max_from_proj(t1, axis_proj);
            let (min_r2, max_r2) = EntityDrawable::get_min_max_from_proj(t2, axis_proj);

            if !(max_r2 >= min_r1 && max_r1 >= min_r2) {
                return false;
            }
        }

        for a in 0..ncols {
            let b = (a + 1) % ncols;
            let axis_proj: Vector2<f64> = t2.column(b) - t2.column(a);
            let axis_proj: Vector2<f64> = Vector2::new(-axis_proj.y, axis_proj.x).normalize();

            let (min_r1, max_r1) = EntityDrawable::get_min_max_from_proj(t1, axis_proj);
            let (min_r2, max_r2) = EntityDrawable::get_min_max_from_proj(t2, axis_proj);

            if !(max_r2 >= min_r1 && max_r1 >= min_r2) {
                return false;
            }
        }

        true
    }

    fn transform_triagle(obj: &EntityDrawable, mut t: Matrix2x3<f64>) -> Matrix2x3<f64> {
        let dimm: Vector2<f64> = obj.object.dimentions() / 2.0;
        let dimm: Matrix2x3<f64> = Matrix2x3::from_columns(&[dimm, dimm, dimm]);

        let coords: Matrix2x3<f64> =
            Matrix2x3::from_columns(&[obj.pos.coords, obj.pos.coords, obj.pos.coords]);

        let rot: Rotation2<f64> = Rotation2::new(-obj.rotation);

        t *= obj.object.scale;

        t -= dimm;
        t = rot * t;
        t += dimm;

        t + coords
    }

    pub fn hit(&self, other: &EntityDrawable) -> bool {
        for triagle in self.object.lst_hit_box.iter() {
            for other_triagle in other.object.lst_hit_box.iter() {
                let triagle2 = EntityDrawable::transform_triagle(self, *triagle);
                let other_triagle = EntityDrawable::transform_triagle(other, *other_triagle);

                if EntityDrawable::triagle_hit(triagle2, other_triagle) {
                    return true;
                }
            }
        }
        false
    }
}
