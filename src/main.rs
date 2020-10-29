pub mod field;
use field::Field;

pub mod display;
use display::Display;

pub mod shaders;

pub mod textures;
use textures::Textures;

use winit::{
    event_loop::ControlFlow,
};

const WINDOW_WIDTH: u32 = 480;
const WINDOW_HEIGHT: u32 = 480;

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 8;

pub type EventLoop = winit::event_loop::EventLoop<()>;
pub type Event<'a> = winit::event::Event<'a, ()>;

#[derive(Debug)]
pub struct Game {
    display: Display,
    textures: Textures,

    field: Field,
    field_populated: bool,

    cursor_position: (u32, u32),
    cursor_pressed: bool,
}

impl Game {
    pub fn new(event_loop: &EventLoop) -> Self {
        let field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);

        let display = Display::new(event_loop);
        let textures = Textures::new(&display);

        Game {
            display,
            textures,

            field,
            field_populated: false,

            cursor_position: (0, 0),
            cursor_pressed: false,
        }
    }

    pub fn run(mut self, event_loop: EventLoop) -> ! {
        self.display.set_visible(true);
        event_loop.run(move |event, _, control_flow| {
            self.event_handler(event, control_flow);
        })
    }

    fn event_handler(&mut self, event: Event, control_flow: &mut ControlFlow) {
        use winit::event::WindowEvent::*;

        match event {
            Event::MainEventsCleared => {
                self.update();
                self.render();
            }

            Event::WindowEvent { event, .. } => match event {
                CloseRequested => *control_flow = ControlFlow::Exit,

                CursorMoved { position, .. } => {
                    self.cursor_position = (position.x as u32, position.y as u32);
                }
                MouseInput { state, button, .. } => {
                    use winit::event::{ElementState, MouseButton};
                    let (x, y) = (
                        (self.cursor_position.0 * self.field.width() as u32 / WINDOW_WIDTH) as u8,
                        (self.cursor_position.1 * self.field.height() as u32 / WINDOW_HEIGHT) as u8,
                    );

                    match state {
                        ElementState::Pressed => match button {
                            MouseButton::Left => self.cursor_pressed = true,
                            MouseButton::Right => self.toggle_flag(x, y),
                            _ => (),
                        }

                        ElementState::Released => {
                            if button == MouseButton::Left {
                                self.cursor_pressed = false;

                                self.reveal(x, y);
                            }
                        }
                    }
                }

                _ => (),
            },

            _ => (),
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn reveal(&mut self, x: u8, y: u8) {
        if !self.field_populated {
            self.field.populate(MINE_COUNT, Some((x, y)), &mut rand::thread_rng());
            self.field_populated = true;
        }

        self.field.reveal(x, y);
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        self.field.toggle_flag(x, y);
    }

    fn update(&self) {}

    fn render(&mut self) {
        let field = &self.field;
        let textures = &self.textures;
        let cursor_position = self.cursor_position;
        let cursor_pressed = self.cursor_pressed;

        self.display.render(move |renderer| {
            renderer.clear((0.5, 0.5, 0.5));

            for x in 0..FIELD_WIDTH {
                for y in 0..FIELD_HEIGHT {
                    let cell = field.get_cell(x, y);
                    let (pressed_x, pressed_y) = (
                        (cursor_position.0 * field.width() as u32 / WINDOW_WIDTH) as u8,
                        (cursor_position.1 * field.height() as u32 / WINDOW_HEIGHT) as u8,
                    );

                    let texture_view = if cell.revealed {
                        if cell.has_mine {
                            &textures.mine
                        } else {
                            &textures.numbers[cell.neighboring_mines as usize]
                        }
                    } else if cell.flagged {
                        &textures.flag
                    } else if cursor_pressed && x == pressed_x && y == pressed_y {
                        &textures.pressed
                    } else {
                        &textures.unrevealed
                    }
                    .create_view(&Default::default());

                    let origin_x = x as f32 / FIELD_WIDTH as f32 * 2.0 - 1.0;
                    let origin_y = 1.0 - (y+1) as f32 / FIELD_HEIGHT as f32 * 2.0;
                    let bounds_x = 2.0 / FIELD_WIDTH as f32;
                    let bounds_y = 2.0 / FIELD_HEIGHT as f32;

                    renderer.draw_rect((origin_x, origin_y), (bounds_x, bounds_y), &texture_view);
                }
            }
        });
    }
}

fn main() {
    let event_loop = EventLoop::with_user_event();
    let game = Game::new(&event_loop);
    game.run(event_loop);
}
