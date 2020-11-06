use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

const BIT_DEPTH: png::BitDepth = png::BitDepth::Eight;
const COLOR_TYPE: png::ColorType = png::ColorType::RGBA;

pub struct Textures {
    _texture_creator: TextureCreator<WindowContext>,

    pub mine: Texture,
    pub unrevealed: Texture,
    pub hover: Texture,
    pub flag: Texture,
    pub hover_flag: Texture,
    pub numbers: [Texture; 9],
}

macro_rules! load {
    ($texture_creator:expr, $path:expr) => {
        Textures::load(&$texture_creator, include_bytes!($path))
    };
}

impl Textures {
    pub fn new(canvas: &WindowCanvas) -> Self {
        let tc = canvas.texture_creator();

        Self {
            mine: load!(tc, "textures/mine.png"),
            unrevealed: load!(tc, "textures/unrevealed.png"),
            hover: load!(tc, "textures/hover.png"),
            flag: load!(tc, "textures/flag.png"),
            hover_flag: load!(tc, "textures/hover_flag.png"),
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
        assert_eq!(info.bit_depth, BIT_DEPTH);
        assert_eq!(info.color_type, COLOR_TYPE);

        let surface = sdl2::surface::Surface::from_data(
            &mut buf,
            info.width,
            info.height,
            4 * info.width,
            sdl2::pixels::PixelFormatEnum::RGBA32,
        )
        .unwrap();
        let mut texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();

        // ensure the texture is filtered
        texture.gl_with_bind(|_, _| unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        });

        texture
    }
}
