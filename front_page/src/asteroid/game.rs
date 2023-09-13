use nalgebra::{Point2, Vector2};
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use super::{data, EntityDrawable};

#[derive(Default, Debug)]
pub struct MouseInput {
    pub pos: Point2<f64>,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug)]
pub enum ButtonState {
    None,
    Pressed,
    Hold,
    Released,
}

#[derive(Debug)]
pub struct KeyBoardInput {
    pub space: ButtonState,
}

impl Default for KeyBoardInput {
    fn default() -> Self {
        Self {
            space: ButtonState::None,
        }
    }
}

#[derive(Default, Debug)]
pub struct UserInput {
    pub mouse: MouseInput,
    pub keyboard: KeyBoardInput,
}

pub struct Game {
    entities: Vec<EntityDrawable>,
    player_index: usize,
    gl: WebGlRenderingContext,
    pub canvas_dim: Vector2<f64>,
    pub input: UserInput,
    rng: rand::rngs::ThreadRng,
    last_shoot: f64,
}

trait GameLogicEntity {
    fn shoud_teleport(&self, canvas_dim: Vector2<f64>) -> bool;
    fn shoud_draw_again(&self, canvas_dim: Vector2<f64>) -> bool;
    fn process_player_acc(&mut self, mouse: Point2<f64>);
    fn process_player_rot(&mut self, mouse: Point2<f64>);
}

impl GameLogicEntity for EntityDrawable {
    fn shoud_teleport(&self, canvas_dim: Vector2<f64>) -> bool {
        let pos = self.pos;
        let width = canvas_dim.x;
        let height = canvas_dim.y;

        (pos.x > width || pos.y > height) || (pos.x < 0.0 || pos.y < 0.0)
    }

    fn shoud_draw_again(&self, canvas_dim: Vector2<f64>) -> bool {
        let pos = self.pos + self.object.dimentions();
        let width = canvas_dim.x;
        let height = canvas_dim.y;

        (pos.x > width || pos.y > height) && !self.delete_on_out_of_bounds
    }

    fn process_player_acc(&mut self, mouse: Point2<f64>) {
        //get center of player
        let player_pos = self.get_pos_center();

        //calcule acceleration vector based on mouse position
        let dir_vector = mouse - player_pos;
        let player_acc: Vector2<f64> = dir_vector.normalize() * 0.0001;
        self.acc = player_acc;
    }

    fn process_player_rot(&mut self, mouse: Point2<f64>) {
        //get center of player
        let player_pos = self.pos.coords + self.object.dimentions() / 2.0;
        let dir_vector = mouse - player_pos;
        //calcule rotation based on mouse position
        let rotation = {
            let rotation = Vector2::new(0.0, -1.0).angle(&dir_vector.coords);
            if dir_vector.x < 0.0 {
                -rotation
            } else {
                rotation
            }
        };
        self.rotation = rotation;
    }
}

impl Game {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let rng = rand::thread_rng();

        let gl: WebGlRenderingContext = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();

        let mut person = EntityDrawable::load_gl(&gl, data::get_ship());

        person.speed = Vector2::new(0.0, 0.0);
        person.pos = Point2::new(0.0, 0.0);
        person.max_speed_sqr = 0.3;
        person.rotation = 0f64.to_radians();
        person.delete_on_out_of_bounds = false;

        let entities = vec![person];

        Self {
            entities,
            gl,
            canvas_dim: Vector2::new(canvas.width() as f64, canvas.height() as f64),
            input: Default::default(),
            player_index: 0, //player is always the first entity
            rng,
            last_shoot: 0.0,
        }
    }

    pub fn set_mouse_input(&mut self, input: MouseInput) {
        self.input.mouse = input;
    }

    // #[allow(dead_code)]
    // fn draw_debug_point(&self, point: Point2<f64>) {
    // self.context.begin_path();
    // self.context
    //     .arc(point.x, point.y, 2.0, 0.0, 2.0 * std::f64::consts::PI)
    //     .unwrap();
    // self.context.fill();
    // }

    // #[allow(dead_code)]
    // fn draw_vector(&self, pos: Point2<f64>, vector: Vector2<f64>) {
    //     self.context.begin_path();
    //     self.context.move_to(pos.x, pos.y);
    //     let vector = pos + vector;
    //     self.context.line_to(vector.x, vector.y);
    //     self.context.stroke();
    //     self.draw_debug_point(vector);
    // }

    fn spawn_bullet(
        player: &mut EntityDrawable,
        input: &UserInput,
        gl: &WebGlRenderingContext,
    ) -> EntityDrawable {
        let player_dim = player.object.dimentions().clone();
        let player_pos = player.pos + player_dim / 2.0;

        let dir_vector = (input.mouse.pos.coords - player_pos.coords).normalize();

        let mut bullet = EntityDrawable::load_gl(gl, data::get_bullet());
        bullet.pos = player_pos + dir_vector * 10.0;
        bullet.speed = dir_vector * 0.1;
        bullet
    }

    // fn draw_text(&self, time: f64, delta: f64) {
    //     let context: &CanvasRenderingContext2d = &self.context;
    //     let player: &Entity = self.entities.get(self.player_index).unwrap();

    //     let mut offset = 1.0;
    //     context
    //         .fill_text(
    //             &format!("FPS  : {:.0}", 1.0 / (delta / 1000.0)),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(&format!("TIME : {:.2}", time / 1000.0), 1.0, 10.0 * offset)
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("POS  : {:?}", player.get_pos()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("VEL  : {:?}", player.get_speed()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("VELM : {:?}", player.get_speed().magnitude()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("ACC  : {:?}", player.get_acc()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("ACCM : {:?}", player.get_acc().magnitude()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     // offset += 1.0;
    // }

    fn random_point(&mut self) -> Point2<f64> {
        Point2::new(
            self.rng.gen_range(0.0..self.canvas_dim.x),
            self.rng.gen_range(0.0..self.canvas_dim.y),
        )
    }

    fn spawn_asteroid(&mut self) -> EntityDrawable {
        let mut asteroid = EntityDrawable::load_gl(&self.gl, data::get_asteroid());

        let pos = self.random_point();
        asteroid.speed = Vector2::new(0.0, 0.0);
        asteroid.pos = pos;
        asteroid.rotation = 0f64.to_radians();
        asteroid.delete_on_out_of_bounds = false;

        asteroid
    }

    pub fn game_loop(&mut self, time: f64, delta: f64) {
        let player = self.entities.get_mut(self.player_index).unwrap();

        // if self.input.mouse.left {
        //     player.process_player_acc(self.input.mouse.pos);
        // } else if player.get_speed().magnitude() > 0.001 {
        //     let arrasto: Vector2<f64> = player.get_speed().normalize() * -0.00007;
        //     player.set_acc(arrasto);
        // } else {
        //     player.set_acc(Vector2::default());
        //     player.set_speed(Vector2::default());
        // }

        if self.input.mouse.left || self.input.mouse.right {
            player.process_player_rot(self.input.mouse.pos);
        }

        if self.input.mouse.right {
            if (time - self.last_shoot) > 200.0 {
                let bullet = Game::spawn_bullet(player, &self.input, &self.gl);
                self.entities.push(bullet);
                self.last_shoot = time;
            }
        }

        if let ButtonState::Pressed = self.input.keyboard.space {
            let entity = self.spawn_asteroid();
            self.entities.push(entity);
        }

        let mut to_be_removed: Vec<usize> = Vec::new();

        //physics loop
        for (i, entity) in self.entities.iter_mut().enumerate() {
            entity.update_physics(delta);

            if entity.shoud_teleport(self.canvas_dim) {
                let mut new_pos = entity.pos.clone();

                new_pos += self.canvas_dim;

                new_pos.x = new_pos.x % self.canvas_dim.x as f64;
                new_pos.y = new_pos.y % self.canvas_dim.y as f64;

                entity.pos = new_pos.coords.abs().into();

                if entity.delete_on_out_of_bounds {
                    to_be_removed.push(i);
                }
            }
        }

        //after each remove the index of the next element is reduced by 1
        let mut indexref: usize = 0;
        for i in to_be_removed {
            self.entities.remove(i - indexref);
            indexref += 1;
        }

        self.gl
            .viewport(0, 0, self.canvas_dim.x as i32, self.canvas_dim.y as i32);
        self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        // //Draw loop
        for entity in self.entities.iter() {
            entity.draw(&self.gl).unwrap();

            if entity.shoud_draw_again(self.canvas_dim) {
                let mut new_pos = entity.pos;
                let size = entity.object.dimentions().clone();

                let diff = new_pos - self.canvas_dim;

                if diff.x > -size.x {
                    new_pos.x = diff.x;
                }

                if diff.y > -size.y {
                    new_pos.y = diff.y;
                }

                entity.draw_position(&self.gl, new_pos).unwrap();
            }
        }

        // {
        //     let _color = ChangeColor::color("red", &self.context);
        //     self.draw_text(time, delta);
        // }
    }
}
