mod app;
mod asteroid;
mod hook;

use app::App;
use asteroid::Entity;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();


    let enti: Entity = asteroid::get_ship().try_into().expect("Falha ao carregar o teste");
    
    log::info!("{:#?}", enti);
}
