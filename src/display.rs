use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

#[derive(Debug)]
pub struct Display {
    window: Window,
}

impl Display {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("wgpu minesweeper")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                    WINDOW_WIDTH,
                    WINDOW_HEIGHT))
            .with_resizable(false)
            .build(&event_loop).unwrap();

        Self {
            window,
        }
    }
}
