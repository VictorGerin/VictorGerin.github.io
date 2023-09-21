use std::mem::size_of;

use nalgebra::{Point2, Rotation2, Vector2};
use rand::{rngs::ThreadRng, Rng};
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

#[derive(Clone, Debug)]
enum GameEntity {
    Asteroid {
        entity: EntityDrawable,
        size: u32,
        hit: bool,
    },
    Bullet {
        entity: EntityDrawable,
        hit: bool,
    },
}

impl GameEntity {
    fn set_hit(&mut self, hit: bool) {
        match self {
            GameEntity::Asteroid { hit: h, .. } => *h = hit,
            GameEntity::Bullet { hit: h, .. } => *h = hit,
            _ => {}
        }
    }
    fn is_hit(&self) -> bool {
        match self {
            GameEntity::Asteroid { hit, .. } => *hit,
            GameEntity::Bullet { hit, .. } => *hit,
            _ => false,
        }
    }
    fn get_entity(&self) -> &EntityDrawable {
        match self {
            GameEntity::Asteroid { entity, .. } => entity,
            GameEntity::Bullet { entity, .. } => entity,
        }
    }

    fn get_entity_mut(&mut self) -> &mut EntityDrawable {
        match self {
            GameEntity::Asteroid { entity, .. } => entity,
            GameEntity::Bullet { entity, .. } => entity,
        }
    }
}

pub struct Game {
    player: EntityDrawable,
    entities: Vec<GameEntity>,
    gl: WebGlRenderingContext,
    pub canvas_dim: Vector2<f64>,
    pub map_dim: Vector2<f64>,
    pub input: UserInput,
    rng: rand::rngs::ThreadRng,
    last_shoot: f64,
    level: u32,
    kill_count: u32,
}

impl EntityDrawable {
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

        let mut player = EntityDrawable::load_gl(&gl, data::get_ship());

        player.speed = Vector2::new(0.0, 0.0);
        player.pos = Point2::new(0.0, 0.0);
        player.max_speed_sqr = 0.3;
        player.rotation = 0f64.to_radians();
        player.delete_on_out_of_bounds = false;

        // let mut person2 = EntityDrawable::load_gl(&gl, data::get_asteroid());
        // person2.object.scale = 6.0;
        // person2.speed = Vector2::new(0.0, 0.0);
        // person2.pos = Point2::new(500.0, 500.0) - person2.object.dimentions() / 2.0;
        // // person2.pos = Game::random_point(&mut rng, Vector2::new(1000.0, 1000.0));
        // person2.max_speed_sqr = 0.3;
        // person2.rotation = 0f64.to_radians();
        // person2.delete_on_out_of_bounds = false;
        // entities.push(person2);

        // gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        Self {
            player,
            entities: vec![],
            gl,
            canvas_dim: Vector2::new(canvas.width() as f64, canvas.height() as f64),
            map_dim: Vector2::new(2000.0, 2000.0),
            input: Default::default(),
            rng,
            last_shoot: 0.0,
            level: 1,
            kill_count: 0,
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
        let a: &[f32] = &[point.x as f32, point.y as f32];
        let t = TesteDraw::new(gl, a, WebGlRenderingContext::POINTS, 6.0);
        t.draw(gl);
    }

    #[allow(dead_code)]
    fn draw_vector(gl: &WebGlRenderingContext, reference: &Vector2<f64>, pos: &Vector2<f64>) {
        let a: &[f32] = &[
            reference.x as f32,
            reference.y as f32,
            pos.x as f32,
            pos.y as f32,
        ];
        let t = TesteDraw::new(gl, a, WebGlRenderingContext::LINES, 0.0);
        t.draw(gl);
    }

    fn spawn_bullet(
        player: &EntityDrawable,
        input: &UserInput,
        gl: &WebGlRenderingContext,
    ) -> GameEntity {
        let nav_center: Vector2<f64> = player.get_pos_center().coords;
        let rot = Rotation2::new(-player.rotation);
        let up = rot * (Vector2::y() * player.object.dimentions().y / 2.0);

        let coors = nav_center + up;

        let player_dim = player.object.dimentions();
        let player_pos = player.pos + player_dim / 2.0;
        let dir_vector = (input.mouse.pos.coords - player_pos.coords).normalize();

        let mut bullet = EntityDrawable::load_gl(gl, data::get_bullet());
        bullet.delete_on_out_of_bounds = true;
        // bullet.object.scale = 5.0;
        bullet.pos = (coors - bullet.object.dimentions() / 2.0).into();
        bullet.speed = dir_vector * 1.0 + player.speed;
        GameEntity::Bullet {
            entity: bullet,
            hit: false,
        }
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

    fn random_point(rng: &mut ThreadRng, map_dim: Vector2<f64>) -> Point2<f64> {
        Point2::new(rng.gen_range(0.0..map_dim.x), rng.gen_range(0.0..map_dim.y))
    }

    fn spawn_asteroid(&mut self) -> GameEntity {
        let mut asteroid = EntityDrawable::load_gl(&self.gl, data::get_asteroid());

        let max_speed = 0.6;

        let pos = Game::random_point(&mut self.rng, self.map_dim);
        asteroid.speed = Vector2::new(
            self.rng.gen_range(0.0..=max_speed),
            self.rng.gen_range(0.0..=max_speed),
        );
        asteroid.pos = pos;
        asteroid.object.scale = 2.0 * 3.0;
        asteroid.rotation = (self.rng.gen_range(0.0..=360.0) as f64).to_radians();
        asteroid.delete_on_out_of_bounds = false;

        GameEntity::Asteroid {
            entity: asteroid,
            size: 2,
            hit: false,
        }
    }

    pub fn game_loop(&mut self, time: f64, delta: f64) {
        self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        if self
            .entities
            .iter()
            .filter(|x| match x {
                GameEntity::Asteroid { .. } => true,
                _ => false,
            })
            .count()
            == 0
        {
            let qtd = 2 + self.level;
            //let qtd = 1;
            for _ in 0..qtd {
                let entity = self.spawn_asteroid();
                self.entities.push(entity);
            }
            self.level += 1;
        }

        // self.draw_vector();

        // let data = [-1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 1.0];
        // let teste = TesteDraw::new(&self.gl, &data, WebGlRenderingContext::LINES, 0f32);
        // teste.draw(&self.gl);

        if self.input.mouse.left {
            self.player.process_player_acc(self.input.mouse.pos);
        } else if self.player.speed.magnitude() > 0.001 {
            let arrasto: Vector2<f64> = self.player.speed.normalize() * -0.00007;
            self.player.acc = arrasto;
        } else {
            self.player.acc = Vector2::default();
            self.player.speed = Vector2::default();
        }

        if self.input.mouse.left || self.input.mouse.right {
            self.player.process_player_rot(self.input.mouse.pos);
        }

        if self.input.mouse.right {
            // if (time - self.last_shoot) > 200.0 {
            let bullet = Game::spawn_bullet(&self.player, &self.input, &self.gl);
            self.entities.push(bullet);
            self.last_shoot = time;
            // }
        }

        // if let ButtonState::Pressed = self.input.keyboard.space {
        //     let entity = self.spawn_asteroid();
        //     self.entities.push(entity);
        // }

        self.player.update_physics(delta);
        //physics loop
        for entity in self.entities.iter_mut().map(|x| x.get_entity_mut()) {
            entity.update_physics(delta);
        }

        let index_bullet: Vec<usize> = self
            .entities
            .iter()
            .enumerate()
            .filter(|(y, x)| match x {
                GameEntity::Bullet { .. } => true,
                _ => false,
            })
            .map(|x| x.0)
            .collect();

        let index_asteroid: Vec<usize> = self
            .entities
            .iter()
            .enumerate()
            .filter(|(y, x)| match x {
                GameEntity::Asteroid { .. } => true,
                _ => false,
            })
            .map(|x| x.0)
            .collect();

        let mut new_asteroid: Vec<GameEntity> = vec![];
        for i in index_asteroid {
            for j in index_bullet.clone() {
                if self.entities[i].is_hit() || self.entities[j].is_hit() {
                    continue;
                }

                if self.entities[i]
                    .get_entity()
                    .hit2(&self.entities[j].get_entity())
                {
                    self.kill_count += 1;
                    log::info!("kill count: {}", self.kill_count);

                    self.entities.get_mut(i).unwrap().set_hit(true);
                    self.entities.get_mut(j).unwrap().set_hit(true);

                    if let GameEntity::Asteroid { size, .. } = self.entities[i] {
                        if size <= 1 {
                            continue;
                        }
                        let size = size - 1;
                        let mut asteroid = self.entities[i].get_entity().clone();
                        asteroid.object.scale = 3.0 * (size as f64);
                        asteroid.speed = Vector2::new(
                            self.rng.gen_range(0.0..=0.6),
                            self.rng.gen_range(0.0..=0.6),
                        );
                        asteroid.rotation = (self.rng.gen_range(0.0..=360.0) as f64).to_radians();

                        new_asteroid.push(GameEntity::Asteroid {
                            entity: asteroid,
                            size: size,
                            hit: false,
                        });

                        let mut asteroid = self.entities[i].get_entity().clone();
                        asteroid.object.scale = 3.0 * (size as f64);
                        asteroid.speed = Vector2::new(
                            self.rng.gen_range(0.0..=0.6),
                            self.rng.gen_range(0.0..=0.6),
                        );
                        asteroid.rotation = (self.rng.gen_range(0.0..=360.0) as f64).to_radians();

                        new_asteroid.push(GameEntity::Asteroid {
                            entity: asteroid,
                            size: size,
                            hit: false,
                        });
                    }

                    continue;
                }
            }

            if self.player.hit2(&self.entities[i].get_entity()) {
                self.kill_count += 0;
                log::info!("player has died");
            }
        }

        new_asteroid.extend(self.entities.clone());
        self.entities = new_asteroid
            .iter()
            .filter(|x| {
                !(x.get_entity().delete_on_out_of_bounds && x.get_entity().shoud_delete()
                    || x.is_hit())
            })
            .cloned()
            .collect();

        for entity in self.entities.iter_mut() {
            entity.get_entity_mut().process_teleport();
        }

        //uncoment for debug colisions triagles
        // for entity in self.entities.iter() {
        //     for triagle in entity.object.lst_hit_box.iter() {
        //         let triagle: Matrix2x3<f64> =
        //             EntityDrawable::transform_triagle(entity, *triagle) / 1000.0;

        //         for i in 0..3 {
        //             let next = (i + 1) % 3;
        //             let a: Vector2<f64> = triagle.column(i).into();
        //             let b: Vector2<f64> = triagle.column(next).into();

        //             Game::draw_vector(&self.gl, &a.into(), &b.into());
        //             Game::draw_debug_point(&self.gl, &a.into());
        //         }
        //     }
        // }

        // //Draw loop
        for entity in self.entities.iter().map(|x| x.get_entity()) {
            entity.draw(&self.gl).unwrap();
            entity.process_redraw(&self.gl).unwrap();
        }

        self.player.draw(&self.gl).unwrap();
        self.player.process_redraw(&self.gl).unwrap();

        // {
        //     self.draw_text(time, delta);
        // }
    }
}
