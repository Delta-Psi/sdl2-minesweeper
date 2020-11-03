pub mod field;
use field::Field;

pub mod textures;
use textures::Textures;

use sdl2::{
    Sdl,
    event::Event,
    render::WindowCanvas,
};

const WINDOW_WIDTH: u32 = 480;
const WINDOW_HEIGHT: u32 = 480;

const FIELD_WIDTH: u8 = 8;
const FIELD_HEIGHT: u8 = 8;
const MINE_COUNT: u16 = 8;

pub struct Game {
    sdl: Sdl,
    canvas: WindowCanvas,
    textures: Textures,

    running: bool,

    field: Field,
    field_populated: bool,
    pressed_cell: Option<(u8, u8)>,
}

impl Game {
    pub fn new() -> Self {
        let field = Field::new(FIELD_WIDTH, FIELD_HEIGHT);

        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window("sdl2 minesweeper", WINDOW_WIDTH, WINDOW_HEIGHT)
            .hidden()
            .build().unwrap();
        let canvas = window.into_canvas()
            .present_vsync()
            .build().unwrap();

        let textures = Textures::new(&canvas);

        Game {
            sdl,
            canvas,
            textures,

            running: false,

            field,
            field_populated: false,
            pressed_cell: None,
        }
    }

    pub fn run(mut self) {
        self.canvas.window_mut().show();
        self.running = true;

        let mut event_pump = self.sdl.event_pump().unwrap();
        while self.running {
            for event in event_pump.poll_iter() {
                self.event_handler(event);
            }

            self.update();
            self.render();
        }
    }

    fn map_window_coords(&self, x: i32, y: i32) -> (u8, u8) {
        (
            (x as u32 * self.field.width() as u32 / WINDOW_WIDTH) as u8,
            (y as u32 * self.field.height() as u32 / WINDOW_HEIGHT) as u8,
        )
    }

    fn event_handler(&mut self, event: Event) {
        use sdl2::mouse::MouseButton;

        match event {
            Event::Quit { .. } => {
                self.running = false;
            }

            Event::MouseMotion { mousestate, x, y, .. } => {
                if mousestate.left() {
                    let (x, y) = self.map_window_coords(x, y);
                    self.pressed_cell = Some((x, y));
                }
            }

            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if mouse_btn == MouseButton::Right {
                    let (x, y) = self.map_window_coords(x, y);
                    self.toggle_flag(x, y)
                }
            }

            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                if mouse_btn == MouseButton::Left {
                    let (x, y) = self.map_window_coords(x, y);
                    self.reveal(x, y);
                    self.pressed_cell = None;
                }
            }

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
        self.canvas.set_draw_color((255, 0, 255));
        self.canvas.clear();

        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
                let cell = self.field.get_cell(x, y);

                let texture = if cell.revealed {
                    if cell.has_mine {
                        &self.textures.mine
                    } else {
                        &self.textures.numbers[cell.neighboring_mines as usize]
                    }
                } else if cell.flagged {
                    &self.textures.flag
                } else if self.pressed_cell.map(|(pressed_x, pressed_y)| x == pressed_x && y == pressed_y)
                    .unwrap_or(false)
                {
                    &self.textures.pressed
                } else {
                    &self.textures.unrevealed
                };

                let origin_x = x as i32 * WINDOW_WIDTH as i32 / FIELD_WIDTH as i32;
                let origin_y = y as i32 * WINDOW_HEIGHT as i32 / FIELD_HEIGHT as i32;
                let bounds_x = WINDOW_WIDTH / FIELD_WIDTH as u32;
                let bounds_y = WINDOW_HEIGHT / FIELD_HEIGHT as u32;

                self.canvas.copy(
                    texture,
                    None,
                    Some((origin_x, origin_y, bounds_x, bounds_y).into())
                ).unwrap();
            }
        }

        self.canvas.present();
    }
}

fn main() {
    let game = Game::new();
    game.run();
}
