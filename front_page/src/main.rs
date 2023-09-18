mod app;
mod asteroid;
mod hook;

use app::App;
use nalgebra::Matrix3x2;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();

    // let m = Matrix3x2::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

    // let json = serde_json::to_string(&m).unwrap();
    // println!("{:#?}", json);

    // let rot = Rotation2::new(1f64.to_radians());
    // let mut temp: Vector2<f64> = Vector2::x() * 30.0;
    // let mut vec: Vec<Vector2<f64>> = vec![];
    // for _ in 0..360 {
    //     let p = (temp + Vector2::new(30.0, 30.0)).into();
    //     vec.push(p);
    //     temp = rot * temp;
    // }
    // let json = serde_json::to_string(&vec).unwrap();
    // println!("{}", json)
}
