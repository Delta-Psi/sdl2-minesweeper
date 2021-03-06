pub mod field;
use field::RevealResult;

pub mod textures;
use textures::Textures;

pub mod audio;
use audio::AudioCallback;

pub mod sfx;
use sfx::SOUND_EFFECTS;

pub mod state;
use state::State;

pub mod particles;
use particles::{Particle, ParticleManager};

pub mod layout;
use layout::FieldLayout;

use std::time::Instant;
use sdl2::{audio::AudioDevice, event::Event, render::WindowCanvas, Sdl};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub struct Game {
    sdl: Sdl,
    canvas: WindowCanvas,
    textures: Textures,

    audio_device: AudioDevice<AudioCallback>,

    running: bool,

    layout: FieldLayout,
    state: State,
    hovering: Option<(u8, u8)>,
    particle_manager: ParticleManager,
}

impl Game {
    pub fn new() -> Self {
        // so we can properly bind the textures
        sdl2::hint::set("SDL_RENDER_DRIVER", "opengl");

        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        video.gl_load_library_default().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

        let window = video
            .window("sdl2 minesweeper", WINDOW_WIDTH, WINDOW_HEIGHT)
            .opengl()
            .hidden()
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas().present_vsync().build().unwrap();

        let textures = Textures::new(&canvas);

        let audio = sdl.audio().unwrap();
        let audio_device = AudioCallback::new_device(&audio);

        let particle_manager = ParticleManager::new(&canvas);

        let state = State::new();
        let layout = FieldLayout::new((WINDOW_WIDTH, WINDOW_HEIGHT), state.field().size());

        Game {
            sdl,
            canvas,
            textures,

            audio_device,

            running: false,

            layout,
            state,
            hovering: None,

            particle_manager,
        }
    }

    pub fn run(mut self) {
        self.canvas.window_mut().show();
        self.audio_device.resume();
        self.running = true;

        let mut last_update = Instant::now();
        let mut event_pump = self.sdl.event_pump().unwrap();
        while self.running {
            for event in event_pump.poll_iter() {
                self.event_handler(event);
            }

            self.render();
            let now = Instant::now();
            self.update((now - last_update).as_secs_f32());
            last_update = now;
        }
    }

    fn map_window_coords(&self, x: i32, y: i32) -> Option<(u8, u8)> {
        let render_bounds = self.layout.field_rect();

        let x = x - render_bounds.x;
        if x < 0 || x >= render_bounds.width() as i32 {
            return None;
        }

        let y = y - render_bounds.y;
        if y < 0 || y >= render_bounds.height() as i32 {
            return None;
        }

        Some((
            (x as u32 * self.state.field().width() as u32 / render_bounds.width() as u32) as u8,
            (y as u32 * self.state.field().height() as u32 / render_bounds.height() as u32) as u8,
        ))
    }

    fn event_handler(&mut self, event: Event) {
        use sdl2::event::WindowEvent;
        use sdl2::mouse::MouseButton;

        match event {
            Event::Quit { .. } => {
                self.running = false;
            }

            Event::Window { win_event, .. } => match win_event {
                WindowEvent::Resized(w, h) => {
                    self.layout.recalculate((w as u32, h as u32), self.state.field().size());
                }

                _ => (),
            }

            Event::MouseMotion { x, y, .. } => {
                self.hovering = self.map_window_coords(x, y);
            }

            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if let Some((x, y)) = self.map_window_coords(x, y) {
                    if mouse_btn == MouseButton::Right {
                        self.state.toggle_flag(x, y);
                    }
                }
            }

            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if mouse_btn == MouseButton::Left {
                    match self.map_window_coords(x, y) {
                        None => (),
                        Some((x, y)) => match self.state.reveal(x, y) {
                            RevealResult::Success(revealed) => {
                                let mut audio_callback = self.audio_device.lock();
                                audio_callback.play_sound_effect(&SOUND_EFFECTS.dig);
                                drop(audio_callback);

                                let render_rect = self.layout.field_rect();

                                use rand::Rng;
                                let mut rng = rand::thread_rng();
                                for (x, y) in revealed {
                                    let px = render_rect.left() as f32 + (x as f32 + 0.5) / self.state.field().width() as f32 * render_rect.width() as f32;
                                    let py = render_rect.top() as f32 + (y as f32 + 0.5) / self.state.field().height() as f32 * render_rect.height() as f32;

                                    for _ in 0 .. rng.gen_range(2, 5) {
                                        let pos = (px, py);

                                        let direction = rng.gen_range(0.0, std::f32::consts::TAU);

                                        let particle = Particle::new(pos, 0.75)
                                            .with_direction(direction, 200.0);
                                        self.particle_manager.spawn(particle);
                                    }
                                }
                            }

                            RevealResult::Mine => {
                                let mut audio_callback = self.audio_device.lock();
                                audio_callback.play_sound_effect(&SOUND_EFFECTS.boom);
                                drop(audio_callback);
                            }

                            _ => (),
                        }
                    }
                }
            }

            _ => (),
        }
    }

    fn update(&mut self, delta: f32) {
        let timer = self.state.timer().as_secs();
        let mines_remaining = self.state.mines_remaining();

        self.canvas
            .window_mut()
            .set_title(&format!(
                "sdl2 minesweeper - {:02}:{:02} - {} remaining",
                timer / 60,
                timer % 60,
                mines_remaining,
            ))
            .unwrap();

        self.particle_manager.update(delta);
    }

    fn render(&mut self) {
        self.canvas.set_draw_color((255, 0, 255));
        self.canvas.clear();

        let field_width = self.state.field().width();
        let field_height = self.state.field().height();

        for x in 0..field_width {
            for y in 0..field_height {
                let cell = self.state.field().get_cell(x, y);

                let texture = if cell.revealed {
                    if cell.has_mine {
                        &self.textures.mine
                    } else {
                        &self.textures.numbers[cell.neighboring_mines as usize]
                    }
                } else if self
                    .hovering
                    .map(|(pressed_x, pressed_y)| x == pressed_x && y == pressed_y)
                    .unwrap_or(false)
                {
                    if cell.flagged {
                        &self.textures.hover_flag
                    } else {
                        &self.textures.hover
                    }
                } else if cell.flagged {
                    &self.textures.flag
                } else {
                    &self.textures.unrevealed
                };

                self.canvas
                    .copy(
                        texture,
                        None,
                        Some(self.layout.cell_rect((x, y))),
                    )
                    .unwrap();
            }
        }

        self.particle_manager.render(&mut self.canvas);

        self.canvas.present();
    }
}

fn main() {
    let game = Game::new();
    game.run();
}
