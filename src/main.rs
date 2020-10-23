pub mod field;
use field::Field;

pub mod display;
use display::Display;

pub mod shaders;

pub mod textures;
use textures::Textures;

pub mod interface;
use interface::{Interface, InterfaceEvent};

use winit::{
    event_loop::ControlFlow,
};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 8;

#[derive(Debug)]
pub enum CustomEvent {
    InterfaceEvent(InterfaceEvent),
}

pub type EventLoop = winit::event_loop::EventLoop<CustomEvent>;
pub type Event<'a> = winit::event::Event<'a, CustomEvent>;
pub type EventLoopProxy = winit::event_loop::EventLoopProxy<CustomEvent>;

#[derive(Debug)]
pub struct Game {
    display: Display,
    interface: Interface,
    textures: Textures,

    field: Field,
    field_populated: bool,
}

impl Game {
    pub fn new(event_loop: &EventLoop) -> Self {
        let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);
        field.populate(MINE_COUNT, None, &mut rand::thread_rng());

        let display = Display::new(event_loop);
        let textures = Textures::new(&display);

        Game {
            display,
            interface: Interface::new(event_loop.create_proxy()),
            textures,

            field,
            field_populated: false,
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

            Event::UserEvent(ref event) => match event {
                CustomEvent::InterfaceEvent(event) => match event {
                    InterfaceEvent::RevealCell(x, y) => {
                        self.reveal(*x, *y);
                    }
                    InterfaceEvent::FlagCell(x, y) => {
                        self.field.flag(*x, *y);
                    }
                }
            }

            _ => (),
        }

        self.interface.on_event(&event, &self.field);
    }

    pub fn reveal(&mut self, x: u8, y: u8) {
        if !self.field_populated {
            self.field.populate(MINE_COUNT, Some((x, y)), &mut rand::thread_rng());
            self.field_populated = true;
        }

        self.field.reveal(x, y);
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

                    let texture_view = if cell.revealed {
                        if cell.has_mine {
                            &textures.mine
                        } else {
                            &textures.numbers[cell.neighboring_mines as usize]
                        }
                    } else if cell.flagged {
                        &textures.flag
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
