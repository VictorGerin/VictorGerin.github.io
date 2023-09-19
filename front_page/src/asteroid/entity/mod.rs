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

    fn min(n1: f64, n2: f64) -> f64 {
        if n1 < n2 {
            n1
        } else {
            n2
        }
    }

    fn max(n1: f64, n2: f64) -> f64 {
        if n1 > n2 {
            n1
        } else {
            n2
        }
    }

    fn triagle_hit(t1: Matrix2x3<f64>, t2: Matrix2x3<f64>) -> bool {
        for a in 0..3 {
            let b = (a + 1) % 3;
            let axis_proj: Vector2<f64> = t1.column(b) - t1.column(a);
            let axis_proj = Vector2::new(-axis_proj.y, axis_proj.x);

            let mut min_r1 = f64::INFINITY;
            let mut max_r1 = -f64::INFINITY;

            for p in 0..3 {
                let p: Vector2<f64> = t1.column(p).into();
                let q = p.dotc(&axis_proj);

                min_r1 = {
                    if q < min_r1 {
                        q
                    } else {
                        min_r1
                    }
                };

                max_r1 = {
                    if q > max_r1 {
                        q
                    } else {
                        max_r1
                    }
                };
            }

            let mut min_r2 = f64::INFINITY;
            let mut max_r2 = -f64::INFINITY;
            for p in 0..3 {
                let p: Vector2<f64> = t2.column(p).into();
                let q = p.dotc(&axis_proj);

                min_r2 = {
                    if q < min_r2 {
                        q
                    } else {
                        min_r2
                    }
                };

                max_r2 = {
                    if q > max_r2 {
                        q
                    } else {
                        max_r2
                    }
                };
            }

            if max_r2 < min_r1 || max_r1 < min_r2 {
                return false;
            }
        }

        true
    }

    pub fn hit(&self, other: &EntityDrawable) -> bool {
        let dimm = self.object.dimentions() / 2.0;
        let dimm = Matrix2x3::from_columns(&[dimm, dimm, dimm]);
        let offset = self.pos.coords;
        let offset = Matrix2x3::from_columns(&[offset, offset, offset]);

        let dimm_other = other.object.dimentions() / 2.0;
        let dimm_other = Matrix2x3::from_columns(&[dimm_other, dimm_other, dimm_other]);
        let offset_other = other.pos.coords;
        let offset_other = Matrix2x3::from_columns(&[offset_other, offset_other, offset_other]);

        for triagle in self.object.lst_hit_box.iter() {
            for other_triagle in other.object.lst_hit_box.iter() {
                let triagle = triagle * self.object.scale;
                let triagle = triagle - dimm;
                let triagle = Rotation2::new(other.rotation) * triagle;
                let triagle = triagle + dimm + offset;
                // log::info!("p1-after = {}", triagle / 1000.0);

                // log::info!("p2 = {}", other_triagle / 1000.0);
                let other_triagle = other_triagle * other.object.scale;
                let other_triagle = other_triagle - dimm_other;
                let other_triagle = Rotation2::new(other.rotation) * other_triagle;
                let other_triagle = other_triagle + dimm_other + offset_other;
                // log::info!("p2-after = {}", other_triagle / 1000.0);

                if EntityDrawable::triagle_hit(triagle, other_triagle) {
                    return true;
                }
            }
        }
        false
    }
}
