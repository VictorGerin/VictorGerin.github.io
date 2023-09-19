mod app;
mod asteroid;
mod hook;

use app::App;
use nalgebra::Vector2;
// use nalgebra::Matrix2x3;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();

    // println!("{:#?}", Vector2::new(1.0, 0.0));

    // let v = Vector2::new(1.0, 0.0);

    // println!("{:#?}", v);
    // println!("{:#?}", v.);
    // let m = Matrix2x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

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
