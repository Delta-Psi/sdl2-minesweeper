use wgpu::Texture;
use crate::display::Display;

#[derive(Debug)]
pub struct Textures {
    pub bomb: Texture,
}

impl Textures {
    pub fn new(display: &Display) -> Self {
        Self {
            bomb: Textures::load(display, include_bytes!("textures/16bit_bomb1.png")),
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
