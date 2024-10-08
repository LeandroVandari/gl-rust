use std::io::Write;

use gl_rust::vertex::Vertex;
use glium::glutin::surface::WindowSurface;
use glium::winit;
use glium::winit::event::WindowEvent;
use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    let mut app: App<glium::index::NoIndices> = App::new(&event_loop);

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let _ = event_loop.run_app(&mut app);
}

struct App<I>
where
    I: for<'a> Into<glium::index::IndicesSource<'a>>,
{
    window: winit::window::Window,
    display: glium::backend::glutin::Display<WindowSurface>,

    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: I,
    program: glium::Program,

    moving: bool,
    last_mouse_pos: (f32, f32),
    offset: [f32; 2],

    last_frame: std::time::Instant,
    last_fps_calc: std::time::Instant,
    frames: u32,

    lines: Vec<gl_rust::vertex::Vertex>,
    lines_in_view: usize,
    starting_lines_amount: usize,
    viewable_coords: f32,
    zoom: i64,
    view: [[f32; 4]; 4],
}

impl App<glium::index::NoIndices> {
    fn new<U>(event_loop: &winit::event_loop::EventLoop<U>) -> Self {
        println!("Creating window...");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Infinite grid")
            .with_inner_size(800, 600)
            .build(event_loop);

        /*let v1 = vertex::vertex(-0.5, -0.5);
                let v2 = vertex::vertex(0.0, 0.5);
                let v3 = vertex::vertex(0.5, -0.25);

                let shape = vec![v1, v2, v3];

                let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                let vert_shader = include_str!("shader.vert");
                let frag_shader = include_str!("shader.frag");
        */

        let starting_lines_amount = 10;
        let lines = gl_rust::generate_line_vertices(starting_lines_amount);
        let vertex_buffer = glium::VertexBuffer::new(&display, &lines).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let vert_shader = include_str!("lines.vert");
        let frag_shader = include_str!("lines.frag");

        let program =
            glium::Program::from_source(&display, vert_shader, frag_shader, None).unwrap();

        Self {
            window,
            display,

            vertex_buffer,
            indices,
            program,

            moving: false,
            last_mouse_pos: (0.0, 0.0),
            offset: [0.0, 0.0],

            last_frame: std::time::Instant::now(),
            last_fps_calc: std::time::Instant::now(),
            frames: 0,

            lines,
            lines_in_view: starting_lines_amount,
            starting_lines_amount,
            viewable_coords: 2.0,
            zoom: 10,
            view: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, -1.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
}

impl<I> winit::application::ApplicationHandler for App<I>
where
    I: for<'a> Into<glium::index::IndicesSource<'a>>,
    for<'b> &'b I: Into<glium::index::IndicesSource<'b>>,
{
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("\nClosing window...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let time_now = std::time::Instant::now();
                let _delta_t = time_now - self.last_frame;
                self.frames += 1;
                self.last_frame = time_now;

                let time_since_fps_calc = time_now - self.last_fps_calc;
                if time_since_fps_calc >= std::time::Duration::from_secs(1) {
                    print!(
                        "\r{:.1}FPS",
                        self.frames as f64 / time_since_fps_calc.as_secs_f64()
                    );

                    let _ = std::io::stdout().flush();
                    self.frames = 0;
                    self.last_fps_calc = time_now;
                }

                let mut frame = self.display.draw();

                let uniforms = glium::uniform! {
                    offset: self.offset,
                    view: self.view,
                    zoom: -(self.zoom as f32)/10.0
                };

                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                frame
                    .draw(
                        &self.vertex_buffer,
                        &self.indices,
                        &self.program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();

                frame.finish().unwrap();
                self.window.request_redraw();
            }
            WindowEvent::Resized(window_size) => {
                self.display.resize(window_size.into());
            }

            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        logical_key: winit::keyboard::Key::Named(key),
                        ..
                    },
                ..
            } => {
                use winit::keyboard::NamedKey;
                match key {
                    NamedKey::Escape => {
                        println!("\nClosing window...");
                        event_loop.exit();
                    }
                    NamedKey::Space => {
                        // println!("{:?}", self.offset);
                    }
                    _ => (),
                }
            }

            WindowEvent::MouseInput {
                button: winit::event::MouseButton::Left,
                state,
                ..
            } => {
                self.moving = matches!(state, winit::event::ElementState::Pressed);
                if self.moving {
                    self.window.set_cursor(winit::window::Cursor::Icon(
                        winit::window::CursorIcon::Grabbing,
                    ))
                } else {
                    self.window.set_cursor(winit::window::Cursor::Icon(
                        winit::window::CursorIcon::Default,
                    ))
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let prev_mouse_pos = self.last_mouse_pos;
                let position_x = position.x as f32;
                let position_y = position.y as f32;
                self.last_mouse_pos = (position_x, position_y);

                if self.moving {
                    let x_diff = position_x - prev_mouse_pos.0;
                    let y_diff = position_y - prev_mouse_pos.1;
                    let size = self.window.inner_size();
                    if position_x < 0.0
                        || position_y < 0.0
                        || position_x > size.width as f32
                        || position_y > size.height as f32
                    {
                        self.moving = false;
                        self.window.set_cursor(winit::window::Cursor::Icon(
                            winit::window::CursorIcon::Default,
                        ));
                        return;
                    }

                    self.offset[0] += x_diff * 2.0 / size.width as f32 * (self.zoom as f32 / 10.0);
                    self.offset[1] += y_diff * 2.0 / size.height as f32 * (self.zoom as f32 / 10.0);

                    for off in self.offset.as_mut() {
                        if *off < -(self.viewable_coords / (self.lines_in_view + 1) as f32)
                            || *off > (self.viewable_coords / (self.lines_in_view + 1) as f32)
                        {
                            let sign = off.signum();
                            *off = -sign * (self.viewable_coords / (self.lines_in_view + 1) as f32)
                                + off.fract();
                        }
                    }
                }
            }
            WindowEvent::MouseWheel {
                delta,
                phase: winit::event::TouchPhase::Moved,
                ..
            } => {
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(diff) => (diff.y / 3.0) as f32,
                };
                self.zoom = 1.max(self.zoom + y as i64);
                let space_between_lines = 2.0 / (self.starting_lines_amount + 1) as f64;
                let mut last_line_coord = self.lines[self.lines.len() - 1].position[1] as f64;
                while last_line_coord < (self.zoom as f64 / 10.0 + space_between_lines) {
                    let next_coord = last_line_coord + space_between_lines;
                    self.lines
                        .extend(gl_rust::create_line_vertices_at_coord(-next_coord as f32));
                    self.lines
                        .extend(gl_rust::create_line_vertices_at_coord(next_coord as f32));
                    self.lines_in_view += 4;
                    last_line_coord = next_coord;
                }
                self.viewable_coords = self.zoom as f32 * 0.2;
                self.lines_in_view = (self.viewable_coords / space_between_lines as f32) as usize;
                self.vertex_buffer = glium::VertexBuffer::new(&self.display, &self.lines).unwrap();
            }
            _ => (),
        }
    }
}
