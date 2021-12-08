mod cubemesh;
mod camera;

// TODO: Clean up imports, maybe spread them out to where they are used
use glium::{
    glutin::{
        event,
        event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
        event_loop::{EventLoop, ControlFlow},  
        window::WindowBuilder, 
        dpi::PhysicalSize, 
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

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const TITLE: &'static str = "Ferrous OpenGL"; 

fn init() -> (glium::Display, EventLoop<()>) {
    let events_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title(TITLE);
    let cb = ContextBuilder::new().with_depth_buffer(24);
    let display = Display::new(wb, cb, &events_loop).unwrap();
    (display, events_loop)
}

fn main() {
    // initialize our event_loop, window, OpenGL context, and display
    let (display, events_loop) = init();
    println!("Window Initialized");

    // begin the event loop to keep the window open
    event_loop(events_loop, display);
}

fn event_loop(event_loop: EventLoop<()>, display: Display) {
    // load the cube buffers
    // TODO: add normals to the cubes and maybe some crude light data
    let positions = VertexBuffer::new(&display, &cubemesh::VERTICES).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &cubemesh::INDICES).unwrap(); 
    println!("Cubemesh Initialized");

    // shaders
    // TODO: move to their own file at some point
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
    println!("Shaders Initialized");

    let mut camera = camera::Camera::new(HEIGHT, WIDTH);
    println!("Camera Initialized");

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        // update camera position
        camera.update();

        match ev {
            // TODO: eventually move to its own "input.rs" file
            Event::DeviceEvent { event, .. } => {
                match event {
                    event::DeviceEvent::Key(KeyboardInput { virtual_keycode, .. }) => {
                        match virtual_keycode.unwrap() {
                            VirtualKeyCode::Escape => {
                                println!("Escape caught");
                                *control_flow = ControlFlow::Exit;
                                return;
                            },
                            _ => {},
                        }
                    },
                    event => camera.parse_input(&event),
                }
            },
            // close the window if asked to
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                },
                _ => {},
            },
            Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => println!("Loop Initialized"),
                _ => {},
            },
            _ => {},
        }
        
        let perspective = camera.perspective_matrix();
        let view = camera.view_matrix();
        let model = [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ 0.0, 0.0, 3.0, 1.0f32 ]
        ];

        let params = DrawParameters {
            depth: glium::Depth { 
                test: draw_parameters::DepthTest::IfLess, 
                write: true,
                .. Default::default() 
            },
            // TODO: Figure this out to avoid rendering inside of cube
            // go through indices to make sure all of the triangles are either CullClockwise or CullCounterClockwise 
            // south, north, top and bottom seem to render properly, 
            // east and west seem to have counter clockwise indices
            //backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        // create the target and clear the color and depth buffers
        // draw the frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);
        target.draw(&positions, &indices, &program, 
            &uniform! { model: model, view: view, perspective: perspective }, &params).unwrap();
        target.finish().unwrap();

    });
}

