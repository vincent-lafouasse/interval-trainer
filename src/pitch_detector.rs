use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::Context;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct MyPitchDetector {
    pub config: MyPitchDetectorConfig,
    pub context: MyPitchDetectorContext,
    pub audio_thread_freq: Arc<AtomicU64>,
    pub ui_thread_freq: Arc<AtomicU64>,
    pub buffer: Vec<f32>,
}

impl MyPitchDetector {
    pub fn new(config: MyPitchDetectorConfig, context: MyPitchDetectorContext) -> Self {
        let freq = Arc::new(AtomicU64::new(0));
        let freq_clone = Arc::clone(&freq);
        MyPitchDetector {
            config,
            context,
            audio_thread_freq: freq,
            ui_thread_freq: freq_clone,
            buffer: Vec::new(),
        }
    }
}

#[derive(Copy, Clone)]
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
    pub input_device: cpal::Device,
    pub stream_config: cpal::StreamConfig,
}

impl MyPitchDetectorContext {
    pub fn new(config: MyPitchDetectorConfig) -> Result<Self, &'static str> {
        let host: cpal::Host = cpal::default_host();
        let input_device: cpal::Device = match host.default_input_device() {
            Some(device) => device,
            None => return Err("no input device available"),
        };
        let stream_config = cpal::StreamConfig {
            channels: config.n_channels,
            sample_rate: cpal::SampleRate(config.sample_rate.into()),
            buffer_size: cpal::BufferSize::Default,
        };

        Ok(Self { host, input_device, stream_config })
    }
}
