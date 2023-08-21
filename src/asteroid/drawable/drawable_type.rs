extern crate nalgebra as na;

use serde::Deserialize;

use na::Point2;
use wasm_bindgen::prelude::*;
use web_sys::*;
use DrawableType::*;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DrawableType {
    DrawableArc {
        point: Point2<f64>,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    },
    DrawableMove {
        point: Point2<f64>,
    },
    DrawableRect {
        point: Point2<f64>,
        point2: Point2<f64>,
    },
    DrawableLineTo {
        point: Point2<f64>,
    },
}

impl super::Drawable for DrawableType {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        _: Point2<f64>,
        _: f64,
        scale: f64,
    ) -> Result<(), JsValue> {
        let point = *match self {
            DrawableArc { point, .. } => point,
            DrawableMove { point } => point,
            DrawableRect { point, .. } => point,
            DrawableLineTo { point, .. } => point,
        };

        let point = point * scale;
        match self {
            DrawableArc {
                radius,
                start_angle,
                end_angle,
                ..
            } => {
                context.arc(point.x, point.y, *radius * scale, *start_angle, *end_angle)?;
            }
            DrawableMove { .. } => {
                context.move_to(point.x, point.y);
            }
            DrawableRect { point2, .. } => {
                let point2 = point2 * scale;
                context.rect(point.x, point.y, point2.x, point2.y);
            }
            DrawableLineTo { .. } => {
                context.line_to(point.x, point.y);
            }
        }

        Ok(())
    }
}
