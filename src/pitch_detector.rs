use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct MyPitchDetector {
    config: MyPitchDetectorConfig,
    audio_thread_freq: Arc<AtomicU64>,
    buffer: Vec<f32>,
}

impl MyPitchDetector {
    pub fn new(config: MyPitchDetectorConfig) -> Self {
        MyPitchDetector {
            config,
            audio_thread_freq: Arc::new(AtomicU64::new(0)),
            buffer: Vec::new(),
        }
    }
}

pub struct MyPitchDetectorConfig {
    pub n_channels: usize,
    pub sample_rate: usize,
    pub buffer_size: usize,
    pub power_threshold: f32,
    pub clarity_threshold: f32,
    pub precision_threshold_cents: i8,
}
