use std::mem::size_of;

use nalgebra::{Point2, Rotation2, Vector2, Vector3};
use rand::Rng;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext};

use super::{data, shader, EntityDrawable};

pub struct TesteDraw {
    prg: WebGlProgram,
    gl_buf: WebGlBuffer,
    count: i32,
    mode: u32,
    psize: f32,
    vertex_pos: u32,
    point_size: u32,
}

impl TesteDraw {
    pub fn new(gl: &WebGlRenderingContext, vertices: &[f32], mode: u32, psize: f32) -> Self {
        let prg = gl.create_program().unwrap();

        let vertex_shader = gl
            .create_shader(WebGlRenderingContext::VERTEX_SHADER)
            .unwrap();
        gl.shader_source(&vertex_shader, shader::get_vertex_shader_teste());
        gl.compile_shader(&vertex_shader);

        let fragment_shader = gl
            .create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
            .unwrap();
        gl.shader_source(&fragment_shader, shader::get_fragment_shader_teste());
        gl.compile_shader(&fragment_shader);

        gl.attach_shader(&prg, &vertex_shader);
        gl.attach_shader(&prg, &fragment_shader);
        gl.link_program(&prg);

        let gl_buf = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buf));
        unsafe {
            let vert_array = js_sys::Float32Array::view(vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        Self {
            mode,
            psize,
            vertex_pos: gl.get_attrib_location(&prg, "vert_position") as u32,
            point_size: gl.get_attrib_location(&prg, "pointSize") as u32,
            prg,
            gl_buf,
            count: vertices.len() as i32 / 2,
        }
    }

    pub fn draw(&self, gl: &WebGlRenderingContext) {
        gl.use_program(Some(&self.prg));

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.gl_buf));

        gl.vertex_attrib_pointer_with_i32(
            self.vertex_pos,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            2 * size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(self.vertex_pos);

        gl.vertex_attrib1f(self.point_size, self.psize);

        gl.draw_arrays(self.mode, 0, self.count);
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
    entities: Vec<EntityDrawable>,
    player_index: usize,
    gl: WebGlRenderingContext,
    pub canvas_dim: Vector2<f64>,
    pub map_dim: Vector2<f64>,
    pub input: UserInput,
    rng: rand::rngs::ThreadRng,
    last_shoot: f64,

    teste: TesteDraw,
}

trait GameLogicEntity {
    fn process_player_acc(&mut self, mouse: Point2<f64>);
    fn process_player_rot(&mut self, mouse: Point2<f64>);
    fn process_teleport(&mut self);
    fn process_redraw(&self, gl: &WebGlRenderingContext) -> Result<(), JsValue>;
    fn shoud_delete(&self) -> bool;
}

impl GameLogicEntity for EntityDrawable {
    fn process_player_acc(&mut self, mouse: Point2<f64>) {
        //get center of player
        let player_pos = self.get_pos_center();

        //calcule acceleration vector based on mouse position
        let dir_vector = mouse - player_pos;
        let player_acc: Vector2<f64> = dir_vector.normalize() * 0.001;
        self.acc = player_acc;
    }

    fn process_player_rot(&mut self, mouse: Point2<f64>) {
        //get center of player
        let player_pos = self.pos.coords + self.object.dimentions() / 2.0;
        let dir_vector = mouse - player_pos;
        //calcule rotation based on mouse position
        self.rotation = {
            let rotation = (dir_vector.y / dir_vector.coords.norm()).acos();
            if dir_vector.x < 0.0 {
                -rotation
            } else {
                rotation
            }
        };
    }

    fn process_teleport(&mut self) {
        if self.pos.x > 1000.0 {
            self.pos.x = -1000.0;
        } else if self.pos.x < -1000.0 {
            self.pos.x = 1000.0;
        }
        if self.pos.y > 1000.0 {
            self.pos.y = -1000.0;
        } else if self.pos.y < -1000.0 {
            self.pos.y = 1000.0;
        }
    }

    fn process_redraw(&self, gl: &WebGlRenderingContext) -> Result<(), JsValue> {
        let dimm = self.object.dimentions();
        let mut pos = self.pos;
        if self.pos.x + dimm.x > 1000.0 {
            pos.x -= 2000.0;
            self.draw_position(gl, pos)?
        }
        if self.pos.y + dimm.y > 1000.0 {
            pos.y -= 2000.0;
            self.draw_position(gl, pos)?
        }
        Ok(())
    }

    fn shoud_delete(&self) -> bool {
        if self.pos.x > 1000.0 {
            return true;
        } else if self.pos.x < -1000.0 {
            return true;
        }
        if self.pos.y > 1000.0 {
            return true;
        } else if self.pos.y < -1000.0 {
            return true;
        }
        false
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

        let mut person2 = EntityDrawable::load_gl(&gl, data::get_ship());

        person2.object.scale = 3.0;
        person2.speed = Vector2::new(0.0, 0.0);
        person2.pos = Point2::new(0.0, 0.0);
        person2.max_speed_sqr = 0.3;
        person2.rotation = 180f64.to_radians();
        person2.delete_on_out_of_bounds = false;

        let entities = vec![person, person2];

        let data = [-1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 1.0];
        let teste = TesteDraw::new(&gl, &data, WebGlRenderingContext::LINES, 0f32);

        Self {
            entities,
            gl,
            canvas_dim: Vector2::new(canvas.width() as f64, canvas.height() as f64),
            map_dim: Vector2::new(2000.0, 2000.0),
            input: Default::default(),
            player_index: 0, //player is always the first entity
            rng,
            last_shoot: 0.0,
            teste,
        }
    }

    pub fn update_input(&mut self) {
        let offset_center: Vector2<f64> = Vector2::new(0.5, 0.5);

        let mut pos: Vector2<f64> =
            self.input.mouse.pos.coords.component_div(&self.canvas_dim) - offset_center;

        pos.y *= -1.0;
        self.input.mouse.pos = pos.component_mul(&self.map_dim).into();
    }

    #[allow(dead_code)]
    fn draw_debug_point(gl: &WebGlRenderingContext, point: &Point2<f64>) {
        let mut bullet = EntityDrawable::load_gl(&gl, data::get_bullet());
        bullet.pos = point.clone();
        bullet.draw(&gl).unwrap();
    }

    #[allow(dead_code)]
    fn draw_vector(&self) {
        let t = TesteDraw::new(&self.gl, &[0.5, 0.5], WebGlRenderingContext::POINTS, 20.0);
        t.draw(&self.gl);
        // self.context.begin_path();
        // self.context.move_to(pos.x, pos.y);
        // let vector = pos + vector;
        // self.context.line_to(vector.x, vector.y);
        // self.context.stroke();
        // self.draw_debug_point(vector);
    }

    fn spawn_bullet(
        player: &EntityDrawable,
        input: &UserInput,
        gl: &WebGlRenderingContext,
    ) -> EntityDrawable {
        let nav_center: Vector2<f64> = player.get_pos_center().coords;
        let rot = Rotation2::new(-player.rotation);
        let up = rot * (Vector2::y() * player.object.dimentions().y / 2.0);

        let coors = nav_center + up;

        let player_dim = player.object.dimentions();
        let player_pos = player.pos + player_dim / 2.0;
        let dir_vector = (input.mouse.pos.coords - player_pos.coords).normalize();

        let mut bullet = EntityDrawable::load_gl(gl, data::get_bullet());
        bullet.delete_on_out_of_bounds = true;
        bullet.pos = (coors - bullet.object.dimentions() / 2.0).into();
        bullet.speed = dir_vector * 1.0 + player.speed;
        bullet
    }

    // fn draw_text(&self, time: f64, delta: f64) {
    //     let context: &CanvasRenderingContext2d = &self.context2d;
    //     let player: &EntityDrawable = self.entities.get(self.player_index).unwrap();

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
    //         .fill_text(&format!("POS  : {:?}", player.pos), 1.0, 10.0 * offset)
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(&format!("VEL  : {:?}", player.speed), 1.0, 10.0 * offset)
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("VELM : {:?}", player.speed.magnitude()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(&format!("ACC  : {:?}", player.acc), 1.0, 10.0 * offset)
    //         .unwrap();
    //     offset += 1.0;
    //     context
    //         .fill_text(
    //             &format!("ACCM : {:?}", player.acc.magnitude()),
    //             1.0,
    //             10.0 * offset,
    //         )
    //         .unwrap();
    //     // offset += 1.0;
    // }

    fn random_point(&mut self) -> Point2<f64> {
        Point2::new(
            self.rng.gen_range(0.0..self.map_dim.x),
            self.rng.gen_range(0.0..self.map_dim.y),
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
        // self.gl
        //     .viewport(0, 0, self.canvas_dim.x as i32, self.canvas_dim.y as i32);
        self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        // self.draw_vector();
        self.teste.draw(&self.gl);

        {
            let player = self.entities.get_mut(self.player_index).unwrap();
            player.rotation += 1f64.to_radians();
            player.pos = self.input.mouse.pos;
            // if self.input.mouse.left {
            //     player.process_player_acc(self.input.mouse.pos);
            // } else if player.speed.magnitude() > 0.001 {
            //     let arrasto: Vector2<f64> = player.speed.normalize() * -0.00007;
            //     player.acc = arrasto;
            // } else {
            //     player.acc = Vector2::default();
            //     player.speed = Vector2::default();
            // }

            // if self.input.mouse.left || self.input.mouse.right {
            //     player.process_player_rot(self.input.mouse.pos);
            // }
        }

        if self.input.mouse.right {
            if (time - self.last_shoot) > 200.0 {
                let player = self.entities.get(self.player_index).unwrap();
                let bullet = Game::spawn_bullet(player, &self.input, &self.gl);
                self.entities.push(bullet);
                self.last_shoot = time;
            }
        }

        if let ButtonState::Pressed = self.input.keyboard.space {
            let entity = self.spawn_asteroid();
            self.entities.push(entity);
        }

        //physics loop
        for entity in self.entities.iter_mut() {
            entity.update_physics(delta);
        }

        self.entities = self
            .entities
            .iter()
            .filter(|x| !x.delete_on_out_of_bounds || !x.shoud_delete())
            .cloned()
            .collect();

        for entity in self.entities.iter_mut() {
            entity.process_teleport();
        }

        for i in 0..self.entities.len() {
            for j in (i + 1)..self.entities.len() {
                // let other = &self.entities[j];
                // self.entities[i].process_collision(other);
                let color = {
                    if self.entities[i].hit(&self.entities[j]) {
                        Vector3::new(1.0, 0.0, 0.0)
                    } else {
                        Vector3::new(1.0, 0.0, 1.0)
                    }
                };
                self.entities.get_mut(i).unwrap().color = color;
                self.entities.get_mut(j).unwrap().color = color;
            }
        }

        // //Draw loop
        for entity in self.entities.iter() {
            entity.draw(&self.gl).unwrap();
            entity.process_redraw(&self.gl).unwrap();
        }

        // {
        //     self.draw_text(time, delta);
        // }
    }
}
