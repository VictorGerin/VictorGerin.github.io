mod app;
mod asteroid;
mod hook;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();

    // let rot = Rotation2::new(90f64.to_radians());
    // let temp: DMatrix<f64> =
    //     DMatrix::from_row_slice(4, 2, &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0]).transpose();
    // temp.row_iter().for_each(|x| {
    //     x.iter().for_each(|x| print!("{:+.1} ", x));
    //     println!();
    // });

    // println!();
    // let mut temp = rot * temp;
    // temp.column_iter_mut()
    //     .for_each(|mut x| x += Vector2::new(10.0, 10.0));

    // temp.row_iter().for_each(|x| {
    //     x.iter().for_each(|x| print!("{:+.1} ", x));
    //     println!();
    // });

    // println!("{:?}", temp);
}
