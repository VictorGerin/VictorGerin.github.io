mod app;
mod asteroid;
mod hook;

use app::App;
use asteroid::Entity;
use nalgebra::{Affine2, DMatrix, Dyn, Matrix, Matrix2, Rotation2, VecStorage, Vector2, U2};

use crate::asteroid::PointMatrix;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();

    // let entity: Entity = asteroid::get_ship()
    //     .try_into()
    //     .expect("err ao carregar nave");

    // println!("{:?}", entity);

    // Affine2::

    // let rot = Rotation2::new(90f64.to_radians());
    // let t = Matrix2::new(1.0, 0.0, 0.0, 1.0);
    // let mut temp: PointMatrix = t * DMatrix::from_row_slice(
    //     4,
    //     2,
    //     &[
    //         0.0f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64, 1.0f64, 1.0f64, 0.0f64,
    //     ],
    // )
    // .transpose();

    // let temp = serde_json::to_string(&(temp.transpose())).unwrap();
    // println!("{}", temp);

    // temp.row_iter().for_each(|x| {
    //     x.iter().for_each(|x| print!("{:+.1} ", x));
    //     println!();
    // });

    // println!();
    // temp.column_iter_mut()
    //     .for_each(|mut x| x += Vector2::new(10.0, 10.0));
    // temp = rot * temp;

    // temp.row_iter().for_each(|x| {
    //     x.iter().for_each(|x| print!("{:+.1} ", x));
    //     println!();
    // });

    // println!("{:?}", temp);
}
