extern crate nalgebra as na;
use super::drawable::*;

use na::{Matrix2x3, Point2, Rotation2, RowVector2, Vector2, Vector3};
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

        for p in 0..points.nrows() {
            let p: Vector2<f64> = points.column(p).into();
            let q = p.dotc(&axis_proj);

            min_r1 = q.min(min_r1);
            max_r1 = q.max(max_r1);
        }
        return (min_r1, max_r1);
    }

    fn triagle_hit(t1: Matrix2x3<f64>, t2: Matrix2x3<f64>) -> bool {
        for a in 0..3 {
            let b = (a + 1) % 3;
            let axis_proj: Vector2<f64> = t1.column(b) - t1.column(a);
            let axis_proj = Vector2::new(-axis_proj.y, axis_proj.x);

            let (min_r1, max_r1) = EntityDrawable::get_min_max_from_proj(t1, axis_proj);
            let (min_r2, max_r2) = EntityDrawable::get_min_max_from_proj(t2, axis_proj);

            if max_r2 < min_r1 || max_r1 < min_r2 {
                return false;
            }
        }

        true
    }

    fn transform_triagle(obj: &EntityDrawable, t: Matrix2x3<f64>) -> Matrix2x3<f64> {
        let dimm:Vector2<f64> = obj.object.dimentions() / 2.0;
        let rot: Rotation2<f64> = Rotation2::new(obj.rotation);
        let mut p1: Vector2<f64> = t.column(0).into();
        let mut p2: Vector2<f64> = t.column(1).into();
        let mut p3: Vector2<f64> = t.column(2).into();

        p1 *= obj.object.scale;
        p2 *= obj.object.scale;
        p3 *= obj.object.scale;

        p1 -= dimm;
        p2 -= dimm;
        p3 -= dimm;

        p1 = rot * p1;
        p2 = rot * p2;
        p3 = rot * p3;

        p1 += dimm + obj.pos.coords;
        p2 += dimm + obj.pos.coords;
        p3 += dimm + obj.pos.coords;

        Matrix2x3::from_columns(&[p1, p2, p3])
    }


    pub fn hit(&self, other: &EntityDrawable) -> bool {

        for triagle in self.object.lst_hit_box.iter() {
            for other_triagle in other.object.lst_hit_box.iter() {

                let triagle2 = EntityDrawable::transform_triagle(self, *triagle);
                let other_triagle = EntityDrawable::transform_triagle(other, *other_triagle);

                log::info!("{:?} {:?}", triagle, triagle2 / 1000.0);

                if EntityDrawable::triagle_hit(triagle2, other_triagle) {
                    return true;
                }
            }
        }
        false
    }
}
