pub mod field;
use field::Field;

pub mod display;
use display::Display;

pub mod shaders;

pub mod textures;
use textures::Textures;

use winit::{
    event::{ElementState, MouseButton},
    event_loop::ControlFlow,
};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 8;

#[derive(Debug)]
pub enum CustomEvent {
    MouseClick {
        position: (u32, u32),
        state: ElementState,
        button: MouseButton,
    },
}

pub type EventLoop = winit::event_loop::EventLoop<CustomEvent>;
pub type Event<'a> = winit::event::Event<'a, CustomEvent>;
pub type EventLoopProxy = winit::event_loop::EventLoopProxy<CustomEvent>;

#[derive(Debug)]
pub struct Game {
    display: Display,
    textures: Textures,

    field: Field,

    event_loop_proxy: EventLoopProxy,
    mouse_input_handler: MouseInputHandler,
}

impl Game {
    pub fn new(event_loop: &EventLoop) -> Self {
        let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);
        field.populate(MINE_COUNT, None, &mut rand::thread_rng());

        let display = Display::new(event_loop);
        let textures = Textures::new(&display);

        Game {
            display,
            textures,

            field,

            event_loop_proxy: event_loop.create_proxy(),
            mouse_input_handler: MouseInputHandler::new(),
        }
    }

    pub fn run(mut self, event_loop: EventLoop) -> ! {
        self.display.set_visible(true);
        event_loop.run(move |event, _, control_flow| {
            self.event_handler(event, control_flow);
        })
    }

    fn event_handler(&mut self, event: Event, control_flow: &mut ControlFlow) {
        use winit::event::WindowEvent;

        match event {
            Event::MainEventsCleared => {
                self.update();
                self.render();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::UserEvent(ref event) => println!("{:?}", event),

            _ => (),
        }

        self.mouse_input_handler
            .on_event(&event, &self.event_loop_proxy);
    }

    fn update(&self) {}

    fn render(&mut self) {
        let field = &self.field;
        let textures = &self.textures;

        self.display.render(move |renderer| {
            renderer.clear((0.5, 0.5, 0.5));

            for x in 0..FIELD_WIDTH {
                for y in 0..FIELD_HEIGHT {
                    let cell = field.get_cell(x, y);

                    let texture_view = if !cell.revealed {
                        &textures.unrevealed
                    } else {
                        &textures.numbers[cell.neighboring_mines as usize]
                    }
                    .create_view(&Default::default());

                    let origin_x = x as f32 / FIELD_WIDTH as f32 * 2.0 - 1.0;
                    let origin_y = y as f32 / FIELD_HEIGHT as f32 * 2.0 - 1.0;
                    let bounds_x = 2.0 / FIELD_WIDTH as f32;
                    let bounds_y = 2.0 / FIELD_HEIGHT as f32;

                    renderer.draw_rect((origin_x, origin_y), (bounds_x, bounds_y), &texture_view);
                }
            }
        });
    }
}

#[derive(Debug)]
struct MouseInputHandler {
    last_mouse_position: (u32, u32),
    cursor_inside: bool,
}

impl MouseInputHandler {
    pub fn new() -> Self {
        Self {
            last_mouse_position: (0, 0),
            cursor_inside: false,
        }
    }

    pub fn on_event(&mut self, event: &Event<'_>, event_loop_proxy: &EventLoopProxy) {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            use winit::event::WindowEvent::*;

            match event {
                CursorMoved { position, .. } => {
                    self.last_mouse_position = (position.x as u32, position.y as u32);
                }

                CursorEntered { .. } => self.cursor_inside = true,
                CursorLeft { .. } => self.cursor_inside = false,

                MouseInput { button, state, .. } => {
                    if self.cursor_inside {
                        event_loop_proxy
                            .send_event(CustomEvent::MouseClick {
                                position: self.last_mouse_position,
                                state: *state,
                                button: *button,
                            })
                            .unwrap();
                    }
                }

                _ => (),
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::with_user_event();
    let game = Game::new(&event_loop);
    game.run(event_loop);
}
