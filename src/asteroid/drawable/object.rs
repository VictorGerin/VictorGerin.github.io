use super::*;
use na::{Point2, Rotation2};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Object {
    dimentions: Vector2<f64>,
    lst: Vec<DrawableType>,
    #[serde(default)]
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
        let mut ship: Object = serde_json::from_str(data)?;

        let mut lst_vec_point: Vec<Point2<f64>> = Vec::with_capacity(ship.lst.len() * 2);

        for drawable_type in ship.lst.iter() {
            match drawable_type {
                DrawableType::DrawableLineTo { point } => {
                    lst_vec_point.push(Point2::new(point.x, point.y));
                }
                DrawableType::DrawableMove { point } => {
                    lst_vec_point.push(Point2::new(point.x, point.y));
                }
                DrawableType::DrawableArc {
                    point,
                    radius,
                    start_angle,
                    end_angle,
                } => {
                    let ref_axis = Vector2::new(1.0, 0.0);
                    let rad_var = 1f64.to_radians();
                    let rot = Rotation2::new(rad_var);
                    let mut vec_dir: Vector2<f64> =
                        Rotation2::new(*start_angle) * ref_axis * *radius;
                    {
                        let mut end_angle = *end_angle;
                        while end_angle > *start_angle {
                            end_angle -= rad_var;
                            let current = point + vec_dir;
                            vec_dir = rot * vec_dir;
                            lst_vec_point.push(Point2::new(current.x, current.y));
                        }
                    }
                    let vec_dir: Vector2<f64> = Rotation2::new(*end_angle) * ref_axis * *radius;
                    let current = point + vec_dir;
                    lst_vec_point.push(Point2::new(current.x, current.y));
                }
                _ => todo!(),
            };
        }
        ship.lst_vec_point = lst_vec_point;
        Ok(ship)
    }
}

impl DrawableCollection<DrawableType> for Object {
    fn get_lst(&self) -> &Vec<DrawableType> {
        &self.lst
    }
}

impl Drawable for Object {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        offset: Point2<f64>,
        rotation: f64,
        _: f64,
    ) -> Result<(), JsValue> {
        context.begin_path();

        let scale = self.scale;
        let center = self.dimentions() / 2.0;

        // let transform: Matrix2<f64> = Matrix2::new_scaling(scale);
        // let transform: Matrix2<f64> = transform * Matrix2::new_translation(&offset);
        // let transform: Matrix2<f64> = transform * Rotation2::new(rotation);

        // Matrix2::new_scaling(scale)
        //     * Matrix2::new_translation(offset.coords)
        //     * Matrix2::new_rotation(rotation);

        //executa offset
        context.translate(offset.x, offset.y)?;

        //executa rotação
        context.translate(center.x, center.y)?;
        context.rotate(rotation)?;
        context.translate(-center.x, -center.y)?;

        let first = self.lst_vec_point[0] * scale;
        context.move_to(first.x, first.y);
        for point in self.lst_vec_point[1..].iter() {
            let point = point * scale;
            context.line_to(point.x, point.y);
        }

        // (self as &dyn DrawableCollection<DrawableType>).draw(context, offset, rotation, scale)?;
        context.stroke();

        context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;

        Ok(())
    }
}
