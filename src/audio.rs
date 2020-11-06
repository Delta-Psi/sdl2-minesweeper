use sdl2::{AudioSubsystem, audio::AudioDevice};

pub const SAMPLE_RATE: u32 = 44100;
const CHANNEL_COUNT: u8 = 1;
const AUDIO_BUFFER_SIZE: u16 = 512;

#[derive(Debug)]
pub struct AudioCallback {
    //sound_effects: Vec<&'static [i16]>,
    sound_effect: &'static [i16],
}

impl AudioCallback {
    pub fn new_device(audio: &AudioSubsystem) -> AudioDevice<AudioCallback> {
        audio.open_playback(
            None,
            &sdl2::audio::AudioSpecDesired {
                freq: Some(SAMPLE_RATE as i32),
                channels: Some(CHANNEL_COUNT),
                samples: Some(AUDIO_BUFFER_SIZE),
            },
            |_| {
                AudioCallback::new()
            },
        ).unwrap()
    }

    fn new() -> Self {
        Self {
            //sound_effect: Vec::new(),
            sound_effect: &[],
        }
    }

    pub fn play_sound_effect(&mut self, sound_effect: &'static [i16]) {
        self.sound_effect = sound_effect;
    }
}

impl sdl2::audio::AudioCallback for AudioCallback {
    type Channel = i16;

    fn callback(&mut self, samples: &mut [i16]) {
        let end = samples.len().min(self.sound_effect.len());

        for i in 0..end {
            samples[i] = self.sound_effect[i];
        }
        self.sound_effect = &self.sound_effect[end..];
        for i in end..samples.len() {
            samples[i] = 0;
        }
    }
}
