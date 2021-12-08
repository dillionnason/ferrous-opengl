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

    println!("Loop Start");
    event_loop.run(move |ev, _, control_flow| {
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
                            _ => return,
                        }
                    },
                    event::DeviceEvent::MouseMotion { delta, .. } => {
                        let (x, y) = delta;
                        let x = x / 100f64;
                        let y = y / 100f64;
                        camera.update(&x, &y);
                    },
                    _ => return,
                }
            },
            // close the window if asked to
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
            _ => {},
            //ev => camera.parse_input(&ev),
        }
        
        let perspective = camera.perspective_matrix();
        let view = camera.view_matrix();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);


        // perspective, model, and view matrices
        //let perspective = {
        //    let (width, height) = target.get_dimensions();
        //    let aspect_ratio = height as f32 / width as f32;
        //    
        //    let fov: f32 = 3.141592 / 3.0;
        //    let zfar = 1024.0;
        //    let znear = 0.1;

        //    let f = 1.0 / (fov / 2.0).tan();

        //    [
        //        [ f * aspect_ratio  , 0.0,              0.0              ,  0.0 ],
        //        [       0.0         ,  f ,              0.0              ,  0.0 ],
        //        [       0.0         , 0.0,  (zfar+znear)/(zfar-znear)    ,  1.0 ],
        //        [       0.0         , 0.0, -(2.0*zfar*znear)/(zfar-znear),  0.0 ],
        //    ]
        //};

        let model = [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ 0.0, 0.0, 3.0, 1.0f32 ]
        ];

        //let view = view_matrix(&[2.0, -1.0, 1.0], &[dx, dy, 1.0], &[0.0, 1.0, 0.0]);

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

//fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
//    let f = {
//        let f = direction;
//        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
//        let len = len.sqrt();
//        [f[0] / len, f[1] / len, f[2] / len]
//    };
//
//    let s = [up[1] * f[2] - up[2] * f[1],
//             up[2] * f[0] - up[0] * f[2],
//             up[0] * f[1] - up[1] * f[0]];
//
//    let s_norm = {
//        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
//        let len = len.sqrt();
//        [s[0] / len, s[1] / len, s[2] / len]
//    };
//
//    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
//             f[2] * s_norm[0] - f[0] * s_norm[2],
//             f[0] * s_norm[1] - f[1] * s_norm[0]];
//
//    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
//             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
//             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
//
//    [
//        [s_norm[0], u[0], f[0], 0.0],
//        [s_norm[1], u[1], f[1], 0.0],
//        [s_norm[2], u[2], f[2], 0.0],
//        [p[0], p[1], p[2], 1.0],
//    ]
//}
