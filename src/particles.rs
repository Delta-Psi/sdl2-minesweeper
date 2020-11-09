use sdl2::render::{WindowCanvas, Texture};
use std::cell::RefCell;

const PARTICLE_SIZE: u32 = 8;
//const PARTICLE_COLOR: (u8, u8, u8) = (0x2e, 0x53, 0x72);
const PARTICLE_COLOR: (u8, u8, u8) = (0xff, 0xff, 0xff);

const PARTICLE_GRAVITY: f32 = 1000.0;

const MAX_PARTICLE_COUNT: usize = 256;

#[derive(Debug)]
pub struct Particle {
    pub pos: (f32, f32),
    pub vel: (f32, f32),

    pub rot: f32,
    pub angular_vel: f32,

    pub max_lifetime: f32,
    pub lifetime: f32,
}

impl Particle {
    pub fn update(&mut self, delta: f32) {
        self.pos.0 += delta * self.vel.0;
        self.pos.1 += delta * (self.vel.1 + delta*0.5*PARTICLE_GRAVITY);

        self.vel.1 += delta * PARTICLE_GRAVITY;

        self.rot += delta * self.angular_vel;

        self.lifetime -= delta;
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }

    pub fn alpha(&self) -> f32 {
        self.lifetime / self.max_lifetime
    }
}

pub struct ParticleManager {
    particles: Vec<Particle>,

    texture: RefCell<Texture>,
}

fn create_texture(canvas: &WindowCanvas) -> Texture {
    let mut pixel_data = [0xff, 0xff, 0xff];
    let surface = sdl2::surface::Surface::from_data(
        &mut pixel_data,
        1, 1,
        3,
        sdl2::pixels::PixelFormatEnum::RGB888,
    ).unwrap();
    let mut texture = canvas.texture_creator()
        .create_texture_from_surface(&surface).unwrap();
    let (r, g, b) = PARTICLE_COLOR;
    texture.set_color_mod(r, g, b);
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);

    texture
}

impl ParticleManager {
    pub fn new(canvas: &WindowCanvas) -> Self {
        Self {
            particles: Vec::new(),

            texture: RefCell::new(create_texture(canvas)),
        }
    }

    pub fn spawn(&mut self, p: Particle) {
        if self.particles.len() < MAX_PARTICLE_COUNT {
            self.particles.push(p);
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.particles.iter_mut().for_each(|p| p.update(delta));
        self.particles.retain(|p| !p.is_dead());
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for p in &self.particles {
            let x = p.pos.0 as i32 - PARTICLE_SIZE as i32/2;
            let y = p.pos.1 as i32 - PARTICLE_SIZE as i32/2;

            let mut texture = self.texture.borrow_mut();
            texture.set_alpha_mod((p.alpha() * 255.0) as u8);

            canvas.copy_ex(
                &*texture,
                None,
                Some((x, y, PARTICLE_SIZE, PARTICLE_SIZE).into()),
                p.rot as f64 / std::f64::consts::TAU * 360.0, None,
                false, false,
            ).unwrap();
        }
    }
}
