use bevy::prelude::*;
use std::sync::Arc;

/// Categorized procedural sound effects for real-time gameplay feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEffect {
    LaserShot,
    Impact,
    BuildTurret,
    RefugeAlarm,
}

/// Event triggered by gameplay systems to request audio playback.
#[derive(Event, Debug, Clone, Copy)]
pub struct PlaySoundEvent(pub SoundEffect);

/// Pre-synthesized audio asset handles stored in memory.
#[derive(Resource)]
pub struct SoundHandles {
    pub laser: Handle<AudioSource>,
    pub impact: Handle<AudioSource>,
    pub build: Handle<AudioSource>,
    pub alarm: Handle<AudioSource>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEvent>()
            .add_systems(Startup, setup_audio_assets)
            .add_systems(Update, play_sound_event_system);
    }
}

/// Generates a standard 16-bit PCM Mono WAV byte buffer in memory.
pub fn generate_wav_bytes(samples: &[f32], sample_rate: u32) -> Vec<u8> {
    let num_samples = samples.len() as u32;
    let byte_rate = sample_rate * 2; // 16-bit mono = 2 bytes per sample
    let data_len = num_samples * 2;
    let total_len = 36 + data_len;

    let mut buf = Vec::with_capacity(44 + data_len as usize);
    // RIFF header
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&total_len.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    // fmt subchunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); // subchunk size (16 for PCM)
    buf.extend_from_slice(&1u16.to_le_bytes()); // audio format (1 = PCM)
    buf.extend_from_slice(&1u16.to_le_bytes()); // mono
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes()); // block align
    buf.extend_from_slice(&16u16.to_le_bytes()); // 16 bits per sample
    // data subchunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_len.to_le_bytes());
    for &sample in samples {
        let val = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        buf.extend_from_slice(&val.to_le_bytes());
    }
    buf
}

fn synth_laser(sample_rate: u32) -> Vec<f32> {
    let duration = 0.12;
    let total_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(total_samples);
    let mut phase = 0.0;

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        // Pitch drop: 900Hz down to 220Hz
        let freq = 900.0 * (1.0 - progress * 0.75);
        phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;
        // Exponential decay envelope
        let env = (-progress * 6.0).exp();
        samples.push(phase.sin() * env * 0.45);
    }
    samples
}

fn synth_impact(sample_rate: u32) -> Vec<f32> {
    let duration = 0.14;
    let total_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(total_samples);
    let mut phase = 0.0;
    let mut seed = 123456789u32;

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        // Low thump: 130Hz decaying to 45Hz
        let freq = 130.0 * (1.0 - progress * 0.65);
        phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;

        // Linear congruential pseudorandom noise for crunch
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((seed >> 16) as f32 / 32768.0) - 1.0;

        let env = (-progress * 8.0).exp();
        let sample = (phase.sin() * 0.6 + noise * 0.4) * env * 0.4;
        samples.push(sample);
    }
    samples
}

fn synth_build(sample_rate: u32) -> Vec<f32> {
    let duration = 0.10;
    let total_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(total_samples);
    let mut phase = 0.0;

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = if t < 0.05 { 520.0 } else { 780.0 };
        phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;
        let env = 1.0 - (t / duration);
        samples.push(phase.sin() * env * 0.35);
    }
    samples
}

fn synth_alarm(sample_rate: u32) -> Vec<f32> {
    let duration = 0.18;
    let total_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(total_samples);
    let mut phase = 0.0;

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 880.0;
        phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;
        // Square wave overtone
        let val = if phase.sin() > 0.0 { 0.4 } else { -0.4 };
        let env = (-t * 3.0).exp();
        samples.push(val * env * 0.35);
    }
    samples
}

fn setup_audio_assets(mut commands: Commands, mut audio_assets: ResMut<Assets<AudioSource>>) {
    const SAMPLE_RATE: u32 = 22050;

    let laser_wav = generate_wav_bytes(&synth_laser(SAMPLE_RATE), SAMPLE_RATE);
    let impact_wav = generate_wav_bytes(&synth_impact(SAMPLE_RATE), SAMPLE_RATE);
    let build_wav = generate_wav_bytes(&synth_build(SAMPLE_RATE), SAMPLE_RATE);
    let alarm_wav = generate_wav_bytes(&synth_alarm(SAMPLE_RATE), SAMPLE_RATE);

    let laser = audio_assets.add(AudioSource {
        bytes: Arc::from(laser_wav.into_boxed_slice()),
    });
    let impact = audio_assets.add(AudioSource {
        bytes: Arc::from(impact_wav.into_boxed_slice()),
    });
    let build = audio_assets.add(AudioSource {
        bytes: Arc::from(build_wav.into_boxed_slice()),
    });
    let alarm = audio_assets.add(AudioSource {
        bytes: Arc::from(alarm_wav.into_boxed_slice()),
    });

    commands.insert_resource(SoundHandles {
        laser,
        impact,
        build,
        alarm,
    });
}

fn play_sound_event_system(
    mut commands: Commands,
    mut events: EventReader<PlaySoundEvent>,
    sounds: Option<Res<SoundHandles>>,
) {
    let Some(sounds) = sounds else {
        return;
    };

    for event in events.read() {
        let handle = match event.0 {
            SoundEffect::LaserShot => &sounds.laser,
            SoundEffect::Impact => &sounds.impact,
            SoundEffect::BuildTurret => &sounds.build,
            SoundEffect::RefugeAlarm => &sounds.alarm,
        };

        commands.spawn((AudioPlayer(handle.clone()), PlaybackSettings::DESPAWN));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wav_generation_header() {
        let samples = vec![0.0, 0.5, -0.5, 0.0];
        let bytes = generate_wav_bytes(&samples, 44100);
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
        assert_eq!(&bytes[12..16], b"fmt ");
        assert_eq!(&bytes[36..40], b"data");
        assert_eq!(bytes.len(), 44 + 4 * 2);
    }

    #[test]
    fn test_audio_source_decodes_successfully() {
        use bevy::audio::Decodable;
        let bytes = generate_wav_bytes(&synth_laser(22050), 22050);
        let source = AudioSource {
            bytes: Arc::from(bytes.into_boxed_slice()),
        };
        let mut decoder = source.decoder();
        assert!(decoder.next().is_some());
    }
}
