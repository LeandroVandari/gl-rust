use glium;
use glium::glutin::surface::WindowSurface;
use glium::winit;
use glium::winit::event::WindowEvent;
use glium::Surface;
use gl_rust::vertex::{self, Vertex};


fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    let mut app: App<glium::index::NoIndices> = App::new(&event_loop);

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    event_loop.run_app(&mut app);
}

struct App<I> where 
I: for<'a> Into<glium::index::IndicesSource<'a>>{
    window: winit::window::Window,
    display: glium::backend::glutin::Display<WindowSurface>,


    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: I,
    program: glium::Program,

    t: f32,
    moving: bool,
    last_mouse_pos: (f64, f64),
}

impl App<glium::index::NoIndices> {
    fn new<U>(event_loop: &winit::event_loop::EventLoop<U>) -> Self {
        println!("Creating window...");
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

        /*let v1 = vertex::vertex(-0.5, -0.5);
        let v2 = vertex::vertex(0.0, 0.5);
        let v3 = vertex::vertex(0.5, -0.25);

        let shape = vec![v1, v2, v3];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vert_shader = include_str!("shader.vert");
        let frag_shader = include_str!("shader.frag");
*/
        let lines = gl_rust::generate_line_vertices();
        let vertex_buffer  = glium::VertexBuffer::dynamic(&display, &lines).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let vert_shader = include_str!("lines.vert");
        let frag_shader = include_str!("lines.frag");

        let program = glium::Program::from_source(&display, vert_shader, frag_shader, None).unwrap();

        Self {
            window,
            display,

            vertex_buffer,
            indices,
            program,

            t: 0.0,
            moving: false,
            last_mouse_pos = (0.0, 0.0)
        }
    }

}

impl<I> winit::application::ApplicationHandler for App<I> where
I: for<'a> Into<glium::index::IndicesSource<'a>>,
for<'b> &'b I:Into<glium::index::IndicesSource<'b>>
 {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window...");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.t += 0.02;
                let x_off = self.t.sin() * 0.5;
                let mut frame = self.display.draw();
                
                let uniforms = glium::uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [ x_off , 0.0, 0.0, 1.0f32],
                    ]

                };

                frame.clear_color(0.0,0.0,0.0,1.0);

                frame.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();

                frame.finish().unwrap();
                self.window.request_redraw();
            }
            WindowEvent::Resized(window_size) => {
                self.display.resize(window_size.into());
            },

            WindowEvent::KeyboardInput {event: winit::event::KeyEvent {logical_key: winit::keyboard::Key::Named(key), ..}, ..} => {
                use winit::keyboard::NamedKey;
                match key {
                    NamedKey::Escape => {
                        println!("Closing window...");
                        event_loop.exit();
                    },
                    _ => {}
                }
            },

            WindowEvent::MouseInput { button: winit::event::MouseButton::Left, state, .. } => {
                self.moving = matches!(state, winit::event::ElementState::Pressed);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let prev_mouse_pos = self.last_mouse_pos;
                self.last_mouse_pos = (position.x, position.y);
                if self.moving {
                    let x_diff = position.x - prev_mouse_pos.0;
                    let y_diff = position.y - prev_mouse_pos.1;
                    for i in 0..20 {
                        self.vertex_buffer[i*2] += x_diff;
                        self.vertex_buffer[20+i*2+1] += y_diff;
                    }

                }
            }
            _ => (),
        }
    }
}
