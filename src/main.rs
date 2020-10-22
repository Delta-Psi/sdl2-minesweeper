pub mod field;
use field::Field;

pub mod display;
use display::Display;

pub mod shaders;

pub mod textures;
use textures::Textures;

use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::Event,
};

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 8;

#[derive(Debug)]
pub struct Game {
    display: Display,
    textures: Textures,

    field: Field,
}

impl Game {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);
        field.populate(MINE_COUNT, None, &mut rand::thread_rng());

        let display = Display::new(&event_loop);
        let textures = Textures::new(&display);

        Game {
            display,
            textures,

            field,
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) -> ! {
        self.display.set_visible(true);
        event_loop.run(move |event, _, control_flow| {
            self.event_handler(event, control_flow);
        })
    }

    fn event_handler(&mut self, event: Event<'_, ()>, control_flow: &mut ControlFlow) {
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

            _ => (),
        }
    }

    fn update(&self) {
    }

    fn render(&mut self) {
        let field = &self.field;
        let textures = &self.textures;

        self.display.render(move |renderer| {
            renderer.clear((0.5, 0.5, 0.5));

            for x in 0 .. FIELD_WIDTH {
                for y in 0 .. FIELD_HEIGHT {
                    let cell = field.get_cell(x, y);

                    let texture_view = if cell.has_mine {
                        &textures.mine
                    } else {
                        &textures.numbers[cell.neighboring_mines as usize]
                    }.create_view(&Default::default());

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

fn main() {
    let event_loop = EventLoop::new();
    let game = Game::new(&event_loop);
    game.run(event_loop);
}
