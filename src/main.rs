pub mod field;
use field::Field;

pub mod display;
use display::Display;

use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::Event,
};

#[derive(Debug)]
pub struct Game {
    display: Display,
}

impl Game {
    pub fn run() -> ! {
        let event_loop = EventLoop::new();

        let game = Game {
            display: Display::new(&event_loop),
        };

        event_loop.run(move |event, _, control_flow| {
            game.event_handler(event, control_flow);
        })
    }

    fn event_handler(&self, event: Event<'_, ()>, control_flow: &mut ControlFlow) {
        use winit::event::WindowEvent;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            _ => (),
        }
    }
}

fn main() {
    const FIELD_WIDTH: u8 = 8;
    const FIELD_HEIGHT: u8 = 8;
    const MINE_COUNT: u16 = 8;

    let mut field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);
    field.populate(MINE_COUNT, None, &mut rand::thread_rng());

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let cell = field.get_cell(x, y);
            if cell.has_mine {
                print!("x")
            } else {
                print!("{}", cell.neighboring_mines);
            }
        }
        println!();
    }

    Game::run()
}
