use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct MyPitchDetector {
    config: MyPitchDetectorConfig,
    audio_thread_freq: Arc<AtomicU64>,
    ui_thread_freq: Arc<AtomicU64>,
    buffer: Vec<f32>,
}

impl MyPitchDetector {
    pub fn new(config: MyPitchDetectorConfig) -> Self {
        let freq = Arc::new(AtomicU64::new(0));
        let freq_clone = Arc::clone(&freq);
        MyPitchDetector {
            config,
            audio_thread_freq: freq,
            ui_thread_freq: freq_clone,
            buffer: Vec::new(),
        }
    }
}

pub struct MyPitchDetectorConfig {
    pub n_channels: u16,
    pub sample_rate: u16,
    pub buffer_size: usize,
    pub power_threshold: f32,
    pub clarity_threshold: f32,
    pub precision_threshold_cents: i8,
}

pub struct MyPitchDetectorContext {
    host: cpal::Host,
    input_device: cpal::Device,
    stream_config: cpal::StreamConfig,
}
