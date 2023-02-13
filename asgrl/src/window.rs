use glium::{
    glutin::{
        dpi,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{WindowBuilder, WindowId},
    },
    Display,
};

#[allow(dead_code)]
pub struct Window {
    width: u32,
    height: u32,
    title: String,
    event_loop: EventLoop<()>,
    display: Display,
    window_id: WindowId,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_inner_size(dpi::PhysicalSize::new(width, height))
            .with_title(title);
        let context_builder = Default::default();
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
        let window_id = display.gl_window().window().id();

        Self {
            width,
            height,
            title: title.to_string(),
            event_loop,
            display,
            window_id,
        }
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = ControlFlow::WaitUntil(next_frame_time);
            if let Event::WindowEvent { event, .. } = event {
                if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
        });
    }
}
