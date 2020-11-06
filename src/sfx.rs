use lazy_static::lazy_static;

const CHANNEL_COUNT: u8 = 1;

pub struct SoundEffects {
    pub dig: Vec<i16>,
}

impl SoundEffects {
    pub fn new() -> Self {
        Self {
            dig: SoundEffects::load(include_bytes!("sfx/dig.ogg")),
        }
    }

    fn load(data: &[u8]) -> Vec<i16> {
        let cursor = std::io::Cursor::new(data);
        let mut reader = lewton::inside_ogg::OggStreamReader::new(cursor).unwrap();

        assert_eq!(reader.ident_hdr.audio_channels, CHANNEL_COUNT);
        assert_eq!(
            reader.ident_hdr.audio_sample_rate,
            crate::audio::SAMPLE_RATE
        );

        let mut buf = Vec::new();
        while let Some(samples) = reader.read_dec_packet_itl().unwrap() {
            buf.extend_from_slice(&samples);
        }

        buf
    }
}

lazy_static! {
    pub static ref SOUND_EFFECTS: SoundEffects = SoundEffects::new();
}
