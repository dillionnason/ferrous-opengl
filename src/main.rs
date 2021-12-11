extern crate image;

mod cubemesh;
mod camera;
use shaders;

use glium::{
    glutin::{
        event,
        event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
        event_loop::{EventLoop, ControlFlow},  
        window::WindowBuilder, 
        dpi::LogicalSize, 
        ContextBuilder
    }, 
    Surface,
    Display, 
    uniform,
    VertexBuffer, 
    index::PrimitiveType, 
    Program, 
    IndexBuffer, DrawParameters, draw_parameters
};

fn main() {
    // initialize our event_loop, window, OpenGL context, and display
    let mut events_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1920, 1080))
        .with_title("Ferrous OpenGL");
    let cb = ContextBuilder::new().with_depth_buffer(24);
    let display = Display::new(wb, cb, &events_loop).unwrap();

    // begin the event loop to keep the window open
    event_loop(events_loop, display);
}

fn event_loop(event_loop: EventLoop<()>, display: Display) {
    let positions = VertexBuffer::new(&display, &cubemesh::VERTICES).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &cubemesh::INDICES).unwrap(); 


    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec4 color;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        out vec4 v_color;

        void main() {
            v_color = color;
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec4 v_color;

        out vec4 color;

        void main() {
            color = v_color;
        }
    "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;
    let mut dx: f32 = 0.0;
    let mut dy: f32 = 0.0;

    let mut camera = camera::Camera::new(); 

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::DeviceEvent { event, .. } => {
                match event {
                    event::DeviceEvent::Key(KeyboardInput { virtual_keycode, .. }) => {
                        match virtual_keycode.unwrap() {
                            VirtualKeyCode::Escape => {
                                *control_flow = ControlFlow::Exit;
                                return;
                            },
                            _ => return,
                        }
                    },
                    event::DeviceEvent::MouseMotion { delta, .. } => {
                        let (x, y) = delta;
                        dx += (x as f32)/100.0;
                        dy -= (y as f32)/100.0;
                    },
                    _ => return,
                }
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => (),
                _ => return,
            },
            _ => (),
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        // draw triangle to the window
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let (height, width) = target.get_dimensions();
        let perspective = camera.perspective_matrix(width as f32, height as f32);

        let model = [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ 0.0, 0.0, 2.0, 1.0f32 ]
        ];

        let params = DrawParameters {
            depth: glium::Depth { 
                test: draw_parameters::DepthTest::IfLess, 
                write: true,
                .. Default::default() 
            },
            //backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let view = camera.view_matrix();

        target.draw(&positions, &indices, &program, 
            &uniform! { model: model, view: view, perspective: perspective }, &params).unwrap();
        target.finish().unwrap();
    });
}

