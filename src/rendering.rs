use glium::{
    Display,
    Program,
    glutin::{
        event_loop::{EventLoop, ControlFlow},
        window::WindowBuilder,
        ContextBuilder,
        event::{WindowEvent, Event},
    },
};
use std::time::{Instant, Duration};

pub fn init() -> (EventLoop<()>, Display, Program) {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("B L O C Q U E S");
    let cb = ContextBuilder::new().with_depth_buffer(24);
    let display = Display::new(wb, cb, &event_loop).unwrap();
    let program = Program::from_source(
        &display,
        String::from_utf8_lossy(include_bytes!("./rendering/shader.vert"))
            .into_owned().as_str(),
        String::from_utf8_lossy(include_bytes!("./rendering/shader.frag"))
            .into_owned().as_str(),
        None,
    ).unwrap();
    (event_loop, display, program)
}

pub fn start(event_loop: EventLoop<()>, loop_fn: Box<dyn Fn(f32, f32)>) {
    let start = Instant::now();
    let mut last_time = start;
    event_loop.run(move |ev, _, control_flow| {
        let now = Instant::now();
        let next_frame_time = now + Duration::from_nanos(16_666_667);
        let total_elapsed = now.duration_since(start).as_secs_f32();
        let elapsed = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        loop_fn(total_elapsed, elapsed);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);
        if let Event::WindowEvent { event, .. } = ev {
            if event == WindowEvent::CloseRequested {
                *control_flow = ControlFlow::Exit;
            }
            return;
        }
    });
}
