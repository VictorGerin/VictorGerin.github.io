use crate::asteroid::entity::Entity;
use nalgebra::{Vector2, Point2};
use rand::Rng;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::data;

struct ChangeColor<'a> {
    last_color: JsValue,
    context: &'a CanvasRenderingContext2d,
}

impl<'a> ChangeColor<'a> {
    fn color(color: &str, context: &'a CanvasRenderingContext2d) -> Self {
        let last_color = context.fill_style();
        let color = JsValue::from(color);
        context.set_fill_style(&color);

        Self {
            last_color,
            context,
        }
    }
}

impl<'a> Drop for ChangeColor<'a> {
    fn drop(&mut self) {
        self.context.set_fill_style(&self.last_color);
    }
}

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
    entities: Vec<Entity>,
    player_index: usize,
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    canvas_dim: Vector2<f64>,
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

impl GameLogicEntity for Entity {
    fn shoud_teleport(&self, canvas_dim: Vector2<f64>) -> bool {
        let pos = self.get_pos();
        let width = canvas_dim.x;
        let height = canvas_dim.y;

        (pos.x > width || pos.y > height) || (pos.x < 0.0 || pos.y < 0.0)
    }

    fn shoud_draw_again(&self, canvas_dim: Vector2<f64>) -> bool {
        let pos = self.get_pos() + self.get_object().dimentions();
        let width = canvas_dim.x;
        let height = canvas_dim.y;

        (pos.x > width || pos.y > height) && !self.get_delete_on_out_of_bounds()
    }

    fn process_player_acc(&mut self, mouse: Point2<f64>) {

        //get center of player
        let player_pos = self.get_pos_center();

        //calcule acceleration vector based on mouse position
        let dir_vector = mouse - player_pos;
        let player_acc: Vector2<f64> = dir_vector.normalize() * 0.0001;
        self.set_acc(player_acc);
    }

    fn process_player_rot(&mut self, mouse: Point2<f64>) {
        //get center of player
        let player_pos = self.get_pos().coords + self.get_object().dimentions() / 2.0;
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
        self.set_rotation(rotation);
    }
}

impl Game {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let rng = rand::thread_rng();

        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut person: Entity = data::get_ship()
            .try_into()
            .expect("Worng json format for Object");

        person.set_speed(Vector2::new(0.0, 0.0));
        person.set_pos(Point2::new(200.0, 200.0));
        person.set_max_speed(0.3);
        person.set_rotation(0f64.to_radians());
        person.set_delete_on_out_of_bounds(false);

        let entities: Vec<Entity> = vec![person];

        Self {
            entities,
            context,
            canvas_dim: Vector2::new(canvas.width() as f64, canvas.height() as f64),
            canvas,
            input: Default::default(),
            player_index: 0, //player is always the first entity
            rng,
            last_shoot: 0.0,
        }
    }

    pub fn set_mouse_input(&mut self, input: MouseInput) {
        self.input.mouse = input;
    }

    #[allow(dead_code)]
    fn draw_debug_point(&self, point: Vector2<f64>) {
        self.context.begin_path();
        self.context
            .arc(point.x, point.y, 10.0, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        self.context.fill();
    }

    #[allow(dead_code)]
    fn draw_vector(context: &CanvasRenderingContext2d, pos: Vector2<f64>, vector: Vector2<f64>) {
        context.begin_path();
        context.move_to(pos.x, pos.y);
        let vector = pos + vector;
        context.line_to(vector.x, vector.y);
        context.stroke();
        // Game::draw_debug_point(vector);
    }

    fn spawn_bullet(player: &mut Entity, input: &UserInput) -> Entity {
        let player_dim = player.get_object().dimentions().clone();
        let player_pos = player.get_pos() + player_dim / 2.0;

        let dir_vector = (input.mouse.pos.coords - player_pos.coords).normalize();

        let mut bullet: Entity = data::get_bullet()
            .try_into()
            .expect("Worng json format for Object");
        bullet.set_pos(player_pos + dir_vector * 10.0);
        bullet.set_speed(dir_vector * 0.1);
        bullet
    }

    fn draw_text(&self, time: f64, delta: f64) {
        let context: &CanvasRenderingContext2d = &self.context;
        let player: &Entity = self.entities.get(self.player_index).unwrap();
        
        let mut offset = 1.0;
        context
            .fill_text(
                &format!("FPS  : {:.0}", 1.0 / (delta / 1000.0)),
                1.0,
                10.0 * offset,
            )
            .unwrap();
        offset += 1.0;
        context
            .fill_text(&format!("TIME : {:.2}", time / 1000.0), 1.0, 10.0 * offset)
            .unwrap();
        offset += 1.0;
        context
            .fill_text(
                &format!("POS  : {:?}", player.get_pos()),
                1.0,
                10.0 * offset,
            )
            .unwrap();
        offset += 1.0;
        context
            .fill_text(
                &format!("VEL  : {:?}", player.get_speed()),
                1.0,
                10.0 * offset,
            )
            .unwrap();
        offset += 1.0;
        context
            .fill_text(
                &format!("VELM : {:?}", player.get_speed().magnitude()),
                1.0,
                10.0 * offset,
            )
            .unwrap();
        offset += 1.0;
        context
            .fill_text(
                &format!("ACC  : {:?}", player.get_acc()),
                1.0,
                10.0 * offset,
            )
            .unwrap();
        offset += 1.0;
        context
        .fill_text(
            &format!("ACCM : {:?}", player.get_acc().magnitude()),
            1.0,
            10.0 * offset,
        )
        .unwrap();
    // offset += 1.0;
    }

    fn random_point(&mut self) -> Point2<f64> {
        Point2::new(self.rng.gen_range(0.0..self.canvas_dim.x), self.rng.gen_range(0.0..self.canvas_dim.y))
    }

    fn spawn_asteroid(&mut self) -> Entity {
        let mut person: Entity = data::get_asteroid()
            .try_into()
            .expect("Worng json format for Object");

        let pos = self.random_point();
        person.set_speed(Vector2::new(0.0, 0.0));
        person.set_pos(pos);
        person.set_rotation(0f64.to_radians());
        person.set_delete_on_out_of_bounds(false);

        person
    }

    pub fn game_loop(&mut self, time: f64, delta: f64) {
        let player = self.entities.get_mut(self.player_index).unwrap();
        self.context.clear_rect(
            0f64,
            0f64,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        if self.input.mouse.left {
            player.process_player_acc(self.input.mouse.pos);
        } else if player.get_speed().magnitude() > 0.001 {
            let arrasto: Vector2<f64> = player.get_speed().normalize() * -0.00007;
            player.set_acc(arrasto);
        } else {
            player.set_acc(Vector2::default());
            player.set_speed(Vector2::default());
        }

        if self.input.mouse.left || self.input.mouse.right {
            player.process_player_rot(self.input.mouse.pos);
        }

        if self.input.mouse.right {
            if (time - self.last_shoot) > 200.0 {
                let bullet = Game::spawn_bullet(
                    player,
                    &self.input,
                );
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
                let mut new_pos = entity.get_pos().clone();

                new_pos += self.canvas_dim;

                new_pos.x = new_pos.x % self.canvas_dim.x as f64;
                new_pos.y = new_pos.y % self.canvas_dim.y as f64;

                entity.set_pos(new_pos.coords.abs().into());

                // entity.set_pos(new_pos.abs());

                if entity.get_delete_on_out_of_bounds() {
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

        // //Draw loop
        for entity in self.entities.iter() {
            entity.draw(&self.context).unwrap();

            if entity.shoud_draw_again(self.canvas_dim)
            {
                let mut new_pos = entity.get_pos();
                let size = entity.get_object().dimentions().clone();

                let diff= new_pos - self.canvas_dim;

                if diff.x > -size.x {
                    new_pos.x = diff.x;
                }

                if diff.y > -size.y {
                    new_pos.y = diff.y;
                }

                entity.draw_position(&self.context, new_pos).unwrap();
            }
        }

        {
            let _color = ChangeColor::color("red", &self.context);
            self.draw_text(
                time,
                delta,
            );
        }
    }

}
