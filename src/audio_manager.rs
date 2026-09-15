use std::collections::HashMap;
use sdl2::mixer::{self, Chunk};


#[derive(Clone,Eq,Hash,PartialEq)]
pub enum SoundType {
    ShipFire,
    ShipHit,
    MonsterFire,
    MonsterHit,
    UfoHit,
    BombHit
}

pub struct AudioManager {
    sounds: HashMap<SoundType, Chunk>,
}

impl AudioManager {

    pub fn new(volume: i32, channels: i32) -> AudioManager {
        mixer::init(mixer::InitFlag::MP3).unwrap();
        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, 2048).unwrap();
        mixer::allocate_channels(channels);
        mixer::Channel::all().set_volume(volume * 128 / 100);

        AudioManager {
            sounds: HashMap::new()
        }
    }

    pub fn load_sound(&mut self, filename: &str, sound: SoundType) {
        match Chunk::from_file(filename) {
            Ok(chunk) => {
                if let Some(old_chunk) = self.sounds.insert(sound, chunk) {
                    drop(old_chunk);
                }
            },
            Err(e) => println!("load sound error: {e}"),
        }
    }

    //set volume for all channels from 0 - 100
    pub fn set_volume(&self, volume: i32) {
        let volume = volume.clamp(0, 100);
        mixer::Channel::all().set_volume(volume * 128 / 100);
    }

    pub fn play_sound(&self, sound: SoundType) {
        if let Some(c) = self.sounds.get(&sound) && let Err(e) = mixer::Channel::all().play(c, 0) {
                println!("audio mixer error: {e}");
        }
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}
