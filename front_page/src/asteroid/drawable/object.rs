use std::mem::size_of;

use super::*;
use na::Point2;
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct Object {
    dimentions: Vector2<f64>,
    lst_vec_point: Vec<Point2<f64>>,
    #[serde(default)]
    scale: f64,
}

impl Object {
    pub fn dimentions(&self) -> Vector2<f64> {
        self.dimentions * self.scale
    }
}

impl TryFrom<&str> for Object {
    type Error = serde_json::Error;
    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let ship: Object = serde_json::from_str(data)?;
        Ok(ship)
    }
}

// impl DrawableCollection<DrawableType> for Object {
//     fn get_lst(&self) -> &Vec<DrawableType> {
//         &self.lst
//     }
// }

impl Drawable for Object {
    fn draw(
        &self,
        gl: &WebGlRenderingContext,
        gl_prg: &WebGlProgram,
        offset: Point2<f64>,
        rotation: f64,
        _: f64,
    ) -> Result<(), JsValue> {
        let points: Vec<f32> = self
            .lst_vec_point
            .iter()
            .flat_map(|point| vec![point.x as f32, point.y as f32])
            .collect();

        let points_buff = gl.create_buffer();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, points_buff.as_ref());
        unsafe {
            let points = js_sys::Float32Array::view(&points);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &points,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let vert_position = gl.get_attrib_location(gl_prg, "vert_position") as u32;
        gl.vertex_attrib_pointer_with_i32(
            vert_position,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            2 * size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(vert_position);

        gl.uniform1f(
            gl.get_uniform_location(gl_prg, "rot").as_ref(),
            rotation as f32,
        );

        gl.draw_arrays(
            WebGlRenderingContext::LINE_LOOP,
            0,
            self.lst_vec_point.len() as i32,
        );

        // context.begin_path();

        // let scale = self.scale;
        // let center = self.dimentions() / 2.0;

        // let transform: Matrix2<f64> = Matrix2::new_scaling(scale);
        // let transform: Matrix2<f64> = transform * Matrix2::new_translation(&offset);
        // let transform: Matrix2<f64> = transform * Rotation2::new(rotation);

        // Matrix2::new_scaling(scale)
        //     * Matrix2::new_translation(offset.coords)
        //     * Matrix2::new_rotation(rotation);

        //executa offset
        // context.translate(offset.x, offset.y)?;

        //executa rotação
        // context.translate(center.x, center.y)?;
        // context.rotate(rotation)?;
        // context.translate(-center.x, -center.y)?;

        // let first = self.lst_vec_point[0] * scale;
        // context.move_to(first.x, first.y);
        // for point in self.lst_vec_point[1..].iter() {
        //     let point = point * scale;
        //     context.line_to(point.x, point.y);
        // }

        // (self as &dyn DrawableCollection<DrawableType>).draw(context, offset, rotation, scale)?;
        // context.stroke();

        // context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;

        Ok(())
    }
}
