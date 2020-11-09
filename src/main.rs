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

use std::time::Instant;
use sdl2::{audio::AudioDevice, event::Event, render::WindowCanvas, Sdl};

const WINDOW_WIDTH: u32 = 480;
const WINDOW_HEIGHT: u32 = 480;

pub struct Game {
    sdl: Sdl,
    canvas: WindowCanvas,
    textures: Textures,

    audio_device: AudioDevice<AudioCallback>,

    running: bool,

    state: State,
    hovering: Option<(u8, u8)>,
    particle_manager: ParticleManager,
}

impl Game {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video
            .window("sdl2 minesweeper", WINDOW_WIDTH, WINDOW_HEIGHT)
            .opengl()
            .hidden()
            .build()
            .unwrap();
        let canvas = window.into_canvas().present_vsync().build().unwrap();

        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

        let textures = Textures::new(&canvas);

        let audio = sdl.audio().unwrap();
        let audio_device = AudioCallback::new_device(&audio);

        let particle_manager = ParticleManager::new(&canvas);

        Game {
            sdl,
            canvas,
            textures,

            audio_device,

            running: false,

            state: State::new(),
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

    fn map_window_coords(&self, x: i32, y: i32) -> (u8, u8) {
        (
            (x as u32 * self.state.field().width() as u32 / WINDOW_WIDTH) as u8,
            (y as u32 * self.state.field().height() as u32 / WINDOW_HEIGHT) as u8,
        )
    }

    fn event_handler(&mut self, event: Event) {
        use sdl2::mouse::MouseButton;

        match event {
            Event::Quit { .. } => {
                self.running = false;
            }

            Event::MouseMotion { x, y, .. } => {
                let (x, y) = self.map_window_coords(x, y);
                self.hovering = Some((x, y));
            }

            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                let (x, y) = self.map_window_coords(x, y);

                if mouse_btn == MouseButton::Right {
                    self.state.toggle_flag(x, y);
                }
            }

            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if mouse_btn == MouseButton::Left {
                    let (x, y) = self.map_window_coords(x, y);

                    match self.state.reveal(x, y) {
                        RevealResult::Success => {
                            let mut audio_callback = self.audio_device.lock();
                            audio_callback.play_sound_effect(&SOUND_EFFECTS.dig);
                            drop(audio_callback);

                            let px = (x as f32 + 0.5) / self.state.field().width() as f32 * WINDOW_WIDTH as f32;
                            let py = (y as f32 + 0.5) / self.state.field().height() as f32 * WINDOW_HEIGHT as f32;
                            self.particle_manager.spawn(Particle {
                                pos: (px, py),
                                vel: (-100.0, -200.0),

                                rot: 0.0,
                                angular_vel: -2.0,

                                max_lifetime: 0.75,
                                lifetime: 0.75,
                            });
                            self.particle_manager.spawn(Particle {
                                pos: (px, py),
                                vel: (100.0, -200.0),

                                rot: 0.0,
                                angular_vel: 2.0,

                                max_lifetime: 0.75,
                                lifetime: 0.75,
                            });
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

                let origin_x = x as i32 * WINDOW_WIDTH as i32 / field_width as i32;
                let origin_y = y as i32 * WINDOW_HEIGHT as i32 / field_height as i32;
                let bounds_x = WINDOW_WIDTH / field_width as u32;
                let bounds_y = WINDOW_HEIGHT / field_height as u32;

                self.canvas
                    .copy(
                        texture,
                        None,
                        Some((origin_x, origin_y, bounds_x, bounds_y).into()),
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
