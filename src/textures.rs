use crate::display::Display;
use wgpu::Texture;

#[derive(Debug)]
pub struct Textures {
    pub mine: Texture,
    pub unrevealed: Texture,
    pub flag: Texture,
    pub numbers: [Texture; 9],
}

impl Textures {
    pub fn new(display: &Display) -> Self {
        Self {
            mine: Textures::load(display, include_bytes!("textures/mine.png")),
            unrevealed: Textures::load(display, include_bytes!("textures/unrevealed.png")),
            flag: Textures::load(display, include_bytes!("textures/flag.png")),
            numbers: [
                Textures::load(display, include_bytes!("textures/0.png")),
                Textures::load(display, include_bytes!("textures/1.png")),
                Textures::load(display, include_bytes!("textures/2.png")),
                Textures::load(display, include_bytes!("textures/3.png")),
                Textures::load(display, include_bytes!("textures/4.png")),
                Textures::load(display, include_bytes!("textures/5.png")),
                Textures::load(display, include_bytes!("textures/6.png")),
                Textures::load(display, include_bytes!("textures/7.png")),
                Textures::load(display, include_bytes!("textures/8.png")),
            ],
        }
    }

    fn load(display: &Display, data: &[u8]) -> Texture {
        let decoder = png::Decoder::new(std::io::Cursor::new(data));
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        // ensure the format is standard
        assert_eq!(info.bit_depth, png::BitDepth::Eight);
        assert_eq!(info.color_type, png::ColorType::RGBA);

        display.create_texture(&buf, info.width, info.height)
    }
}
