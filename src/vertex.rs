use glium;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    is_horizontal: u32,
}

pub const fn vertex(x: f32, y: f32, is_horizontal: u32) -> Vertex {
    Vertex {
        position: [x, y],
        is_horizontal,
    }
}

glium::implement_vertex!(Vertex, position, is_horizontal);
