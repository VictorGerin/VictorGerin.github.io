pub fn get_vertex_shader() -> &'static str {
    include_str!("vertex_shader.glsl")
}

pub fn get_fragment_shader() -> &'static str {
    include_str!("fragment_shader.glsl")
}
