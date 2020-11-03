use sdl2::render::{TextureCreator, Texture, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Textures {
    _texture_creator: TextureCreator<WindowContext>,

    pub mine: Texture,
    pub unrevealed: Texture,
    pub pressed: Texture,
    pub flag: Texture,
    pub numbers: [Texture; 9],
}

macro_rules! load {
    ($texture_creator:expr, $path:expr) => {Textures::load(&$texture_creator, include_bytes!($path))}
}

impl Textures {
    pub fn new(canvas: &WindowCanvas) -> Self {
        let tc = canvas.texture_creator();

        Self {
            mine: load!(tc, "textures/mine.png"),
            unrevealed: load!(tc, "textures/unrevealed.png"),
            pressed: load!(tc, "textures/pressed.png"),
            flag: load!(tc, "textures/flag.png"),
            numbers: [
                load!(tc, "textures/0.png"),
                load!(tc, "textures/1.png"),
                load!(tc, "textures/2.png"),
                load!(tc, "textures/3.png"),
                load!(tc, "textures/4.png"),
                load!(tc, "textures/5.png"),
                load!(tc, "textures/6.png"),
                load!(tc, "textures/7.png"),
                load!(tc, "textures/8.png"),
            ],

            _texture_creator: tc,
        }
    }

    fn load(texture_creator: &TextureCreator<WindowContext>, data: &[u8]) -> Texture {
        let decoder = png::Decoder::new(std::io::Cursor::new(data));
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        // ensure the format is standard
        assert_eq!(info.bit_depth, png::BitDepth::Eight);
        assert_eq!(info.color_type, png::ColorType::RGBA);

        let surface = sdl2::surface::Surface::from_data(
            &mut buf,
            info.width, info.height,
            4 * info.width,
            sdl2::pixels::PixelFormatEnum::RGBA32,
        ).unwrap();
        texture_creator.create_texture_from_surface(surface).unwrap()
    }
}
