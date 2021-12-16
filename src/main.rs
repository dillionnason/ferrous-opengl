mod cubemesh;
mod camera;
mod chunkmesh;
mod debug;

extern crate clock_ticks;
extern crate image;

// TODO: Clean up imports, maybe spread them out to where they are used
use glium::{
    glutin::{
        event,
        event::{Event, WindowEvent},
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
    IndexBuffer, 
    DrawParameters, 
    draw_parameters
};

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;
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
    let mut chunk_mesh = chunkmesh::ChunkMesh::new();
    chunk_mesh.generate_mesh();
    // load the cube buffers
    // TODO: add normals to the cubes and maybe some crude light data
    let positions = VertexBuffer::new(&display, &chunk_mesh.vertices).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &chunk_mesh.indices).unwrap(); 
    println!("Cubemesh Initialized");

    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("textures/grass.png")),
                image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
    println!("Texture Initialized");

    let program = Program::from_source(
        &display, 
        include_str!("shaders/vertex.glsl"), 
        include_str!("shaders/fragment.glsl"),
        None)
        .unwrap();
    println!("Shaders Initialized");

    let mut camera = camera::Camera::new(HEIGHT, WIDTH);
    println!("Camera Initialized");

    let mut debug = debug::Debug::new();

    let mut previous_clock = clock_ticks::precise_time_ns();
   
    display.gl_window().window().set_cursor_grab(true).unwrap();
    display.gl_window().window().set_cursor_visible(false);

    event_loop.run(move |ev, _, control_flow| {

        let now = clock_ticks::precise_time_ns();
        let dt = now - previous_clock;

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);
        
        // update camera position
        camera.update((dt as f32) / 1000000000f32);

        match ev {
            // TODO: eventually move to its own "input.rs" file
            Event::DeviceEvent { event, .. } => {
                match event {
                    //event::DeviceEvent::Key(KeyboardInput { virtual_keycode, .. }) => {
                    //    match virtual_keycode.unwrap() {
                    //        VirtualKeyCode::Escape => {
                    //            *control_flow = ControlFlow::Exit;
                    //            return;
                    //        },
                    //        _ => {},
                    //    }
                    //},
                    event => {
                        camera.parse_input(&event);
                        debug.parse_input(&event, control_flow);
                    }
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

        let _ = display.gl_window().window().set_cursor_position(glium::glutin::dpi::LogicalPosition::new(WIDTH/2f32, HEIGHT/2f32));
        
        let perspective = camera.perspective_matrix();
        let view = camera.view_matrix();
        let model = [
            [ 1.0, 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ -16.0, -32.0, -16.0, 1.0f32 ]
        ];

        let draw_mode = debug.get_draw_state();

        let params = DrawParameters {
            depth: glium::Depth { 
                test: draw_parameters::DepthTest::IfLess, 
                write: true,
                .. Default::default() 
            },
            polygon_mode: draw_mode,
            // TODO: Figure this out to avoid rendering inside of cube
            // go through indices to make sure all of the triangles are either CullClockwise or CullCounterClockwise 
            // south, north, top and bottom seem to render properly, 
            // east and west seem to have counter clockwise indices
            //backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        previous_clock = now;

        // create the target and clear the color and depth buffers
        // draw the frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 1.0, 1.0, 1.0), 1.0);
        target.draw(&positions, &indices, &program, 
            &uniform! { model: model, view: view, perspective: perspective, tex: &texture }, &params).unwrap();
        target.finish().unwrap();
    });
}

