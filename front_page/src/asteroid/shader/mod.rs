pub fn get_vertex_shader() -> &'static str {
    include_str!("Object/vertex_shader.glsl")
}

pub fn get_fragment_shader() -> &'static str {
    include_str!("Object/fragment_shader.glsl")
}

pub fn get_vertex_shader_teste() -> &'static str {
    include_str!("Teste/vertex.glsl")
}

pub fn get_fragment_shader_teste() -> &'static str {
    include_str!("Teste/fragment.glsl")
}
