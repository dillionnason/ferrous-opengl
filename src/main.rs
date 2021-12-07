mod cubemesh;

// TODO: Clean up imports, maybe spread them out to where they are used
use glium::{
    glutin::{
        event,
        event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
        event_loop::{EventLoop, ControlFlow},  
        window::WindowBuilder, 
        dpi::{LogicalSize, Position, LogicalPosition}, 
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

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;
pub const TITLE: &'static str = "Ferrous OpenGL"; 

fn init() -> (glium::Display, EventLoop<()>) {
    let events_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_title(TITLE);
    let cb = ContextBuilder::new().with_depth_buffer(24);
    let display = Display::new(wb, cb, &events_loop).unwrap();
    (display, events_loop)
}

fn main() {
    // initialize our event_loop, window, OpenGL context, and display
    let (display, events_loop) = init();

    // begin the event loop to keep the window open
    event_loop(events_loop, display);
}

fn event_loop(event_loop: EventLoop<()>, display: Display) {
    // load the cube buffers
    // TODO: add normals to the cubes and maybe some crube light data
    let positions = VertexBuffer::new(&display, &cubemesh::VERTICES).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &cubemesh::INDICES).unwrap(); 

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

    // t is used for motion debugging (matrices)
    // dx and dy are used in some really crude raw mouse motion
    let mut t: f32 = -0.5;
    let mut dx: f32 = 0.0;
    let mut dy: f32 = 0.0;
    
    // grab cursor in the window
    display.gl_window().window()
        .set_cursor_grab(true)
        .unwrap();

    event_loop.run(move |ev, _, control_flow| {
        // reset cursor position every loop
        display.gl_window().window()
            .set_cursor_position(Position::Logical(LogicalPosition{x: 0.0, y: 0.0}))
            .unwrap();

        match ev {
            // handles all mouse and keyboard input
            // TODO: eventually move to its own "input.rs" file
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
                    // TODO: change the way mouse motion is handled to get better input
                    // some kind of smoothing needed as well as full 360 degree look
                    // maybe some kind of clamping to prevent the camera from flipping
                    event::DeviceEvent::MouseMotion { delta, .. } => {
                        let (x, y) = delta;
                        dx += (x as f32)/100.0;
                        dy -= (y as f32)/100.0;
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
            _ => (),
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        // used for debugging motion
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        // create the target and clear the color and depth buffers
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // perspective, model, and view matrices
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            
            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [ f * aspect_ratio  , 0.0,              0.0              ,  0.0 ],
                [       0.0         ,  f ,              0.0              ,  0.0 ],
                [       0.0         , 0.0,  (zfar+znear)/(zfar-znear)    ,  1.0 ],
                [       0.0         , 0.0, -(2.0*zfar*znear)/(zfar-znear),  0.0 ],
            ]
        };

        let model = [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ 0.0, 0.0, 2.0, 1.0f32 ]
        ];

        let view = view_matrix(&[2.0, -1.0, 1.0], &[dx, dy, 1.0], &[0.0, 1.0, 0.0]);

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

        // draw the frame
        target.draw(&positions, &indices, &program, 
            &uniform! { model: model, view: view, perspective: perspective }, &params).unwrap();
        target.finish().unwrap();
    });
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
