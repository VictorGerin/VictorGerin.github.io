use std::rc::Rc;

use crate::asteroid::shader;

use super::*;
use na::{Matrix2x3, Matrix2xX, Point2, Vector3};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum DrawMode {
    LineLoop,
    TriangleFan,
    TriangleStrip,
    Lines,
    Triangles,
    Points,
}

impl Default for DrawMode {
    fn default() -> Self {
        Self::LineLoop
    }
}

#[derive(Deserialize, Debug)]
pub struct Object {
    dimentions: Vector2<f64>,
    hit_box: Matrix2xX<f64>,
    hit_box_edge: Matrix2xX<usize>,
    hit_box_obj: Vec<Vec<usize>>,
    lst_vec_point: Vec<Point2<f64>>,
    lst_hit_box: Vec<Matrix2x3<f64>>,
    #[serde(default)]
    scale: f64,
    #[serde(default)]
    buff_loc: Option<usize>,
    #[serde(default)]
    draw_mode: DrawMode,
}

//afim de evitar que o programa seja compilado varias vezes e o mesmo espaço de memoria seja alocado na placa de video
//cria-se uma variavel estatica que vai ser compartilhada entre todas as instancias de ObjectDrawable
static mut GL_PRG: Option<Rc<WebGlProgram>> = None;
static mut GL_BUF: Option<Vec<Option<Rc<WebGlBuffer>>>> = None;

#[derive(Debug, Clone)]
pub struct ObjectDrawable {
    pub lst_hit_box: Vec<Matrix2x3<f64>>,
    pub dimentions: Vector2<f64>,
    pub scale: f64,
    prg: Rc<WebGlProgram>,
    gl_buf: Rc<WebGlBuffer>,
    vertex_count: i32,
    draw_mode: DrawMode,
    pub hit_box: Matrix2xX<f64>,
    pub hit_box_edge: Matrix2xX<usize>,
    pub hit_box_obj: Vec<Vec<usize>>,
}

impl Object {
    fn init_web_gl_program(gl: &WebGlRenderingContext) -> Rc<WebGlProgram> {
        if let Some(prg) = unsafe { GL_PRG.clone() } {
            return prg.clone();
        }

        let vertex_shader = gl
            .create_shader(WebGlRenderingContext::VERTEX_SHADER)
            .unwrap();
        gl.shader_source(&vertex_shader, shader::get_vertex_shader());
        gl.compile_shader(&vertex_shader);
        if !gl
            .get_shader_parameter(&vertex_shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap()
        {
            panic!(
                "Erro ao compilar o vertex shader: {}",
                gl.get_shader_info_log(&vertex_shader).unwrap()
            );
        }

        let fragment_shader = gl
            .create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
            .unwrap();
        gl.shader_source(&fragment_shader, shader::get_fragment_shader());
        gl.compile_shader(&fragment_shader);
        if !gl
            .get_shader_parameter(&fragment_shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap()
        {
            panic!(
                "Erro ao compilar o fragment shader: {}",
                gl.get_shader_info_log(&fragment_shader).unwrap()
            );
        }
        let prg = gl.create_program().unwrap();
        gl.attach_shader(&prg, &vertex_shader);
        gl.attach_shader(&prg, &fragment_shader);
        gl.link_program(&prg);
        if !gl
            .get_program_parameter(&prg, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap()
        {
            panic!(
                "Erro ao linkar o programa: {}",
                gl.get_program_info_log(&prg).unwrap()
            );
        }

        unsafe {
            GL_PRG = Some(Rc::new(prg));
        }

        Object::init_web_gl_program(gl)
    }

    fn create_gl_buf(&self, gl: &WebGlRenderingContext) -> Rc<WebGlBuffer> {
        let points: Vec<f32> = self
            .lst_vec_point
            .iter()
            .flat_map(|point| [point.x as f32, point.y as f32])
            .collect();

        let points_buff = gl.create_buffer();
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, points_buff.as_ref());
        unsafe {
            let points = js_sys::Float32Array::view(&points);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &points,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        Rc::new(points_buff.unwrap())
    }
    fn init_buff(&self, gl: &WebGlRenderingContext) -> Rc<WebGlBuffer> {
        //Se n tiver index cria-se um novo buffer sempre
        if let None = self.buff_loc {
            return self.create_gl_buf(gl);
        }

        //Caso contrario pesquisa p/ saber se já existe um buffer criado
        let index = self.buff_loc.unwrap();

        //recupera a lista de buffers como mutável
        let gl_buf = unsafe {
            if GL_BUF.is_none() {
                GL_BUF = Some(Vec::new());
            }
            GL_BUF.as_mut().unwrap()
        };

        //Se o Vetor n tiver o tamanho do index, vai expandir o vetor até o index
        if index >= gl_buf.len() {
            gl_buf.resize(index + 1, None);
        }

        //Se n existir cria-se um novo buff e guarda no index
        if let None = gl_buf[index] {
            let points_buff = self.create_gl_buf(gl);
            gl_buf[index] = Some(points_buff);
        }

        //retorna o buffer
        gl_buf[index].as_ref().unwrap().clone()
    }

    pub fn load_gl(self, gl: &WebGlRenderingContext) -> ObjectDrawable {
        let prg = Object::init_web_gl_program(gl);
        let gl_buf = self.init_buff(gl);

        ObjectDrawable {
            lst_hit_box: self.lst_hit_box,
            dimentions: self.dimentions,
            scale: self.scale,
            prg,
            gl_buf,
            vertex_count: self.lst_vec_point.len() as i32,
            draw_mode: self.draw_mode,
            hit_box: self.hit_box,
            hit_box_edge: self.hit_box_edge,
            hit_box_obj: self.hit_box_obj,
        }
    }
}

impl TryFrom<&str> for Object {
    type Error = serde_json::Error;
    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let ship: Object = serde_json::from_str(data)?;
        Ok(ship)
    }
}

impl ObjectDrawable {
    pub fn dimentions(&self) -> Vector2<f64> {
        self.dimentions * self.scale
    }
}

impl Drawable for ObjectDrawable {
    fn draw(
        &self,
        gl: &WebGlRenderingContext,
        offset: Point2<f64>,
        rotation: f64,
        color: Vector3<f64>,
    ) -> Result<(), JsValue> {
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.gl_buf));
        let gl_prg = &self.prg;

        gl.use_program(Some(gl_prg));

        let vert_position = gl.get_attrib_location(gl_prg, "vert_position") as u32;
        gl.vertex_attrib_pointer_with_i32(
            vert_position,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(vert_position);

        gl.vertex_attrib1f(
            gl.get_attrib_location(gl_prg, "rot") as u32,
            rotation as f32,
        );

        gl.vertex_attrib2fv_with_f32_array(
            gl.get_attrib_location(gl_prg, "offset") as u32,
            offset.coords.cast().as_slice(),
        );

        gl.vertex_attrib2fv_with_f32_array(
            gl.get_attrib_location(gl_prg, "dimm") as u32,
            self.dimentions().cast().as_slice(),
        );

        gl.vertex_attrib1f(
            gl.get_attrib_location(gl_prg, "scale") as u32,
            self.scale as f32,
        );

        gl.vertex_attrib3fv_with_f32_array(
            gl.get_attrib_location(gl_prg, "color") as u32,
            color.cast().as_slice(),
        );

        let mode = match self.draw_mode {
            DrawMode::LineLoop => WebGlRenderingContext::LINE_LOOP,
            DrawMode::TriangleFan => WebGlRenderingContext::TRIANGLE_FAN,
            DrawMode::Points => WebGlRenderingContext::POINTS,
            _ => WebGlRenderingContext::LINE_LOOP,
        };

        gl.draw_arrays(mode, 0, self.vertex_count);

        Ok(())
    }
}
