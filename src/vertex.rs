use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2]

}

pub const fn vertex(x: f32, y: f32) -> Vertex {
    Vertex {
        position: [x, y]
    
    }
}

glium::implement_vertex!(Vertex, position);
