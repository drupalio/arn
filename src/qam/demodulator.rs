use std::f32::consts::PI;
use complex::Complex;

use qam::Constellation;
use filter::{Filter, Resampler};
use audio::Audio;

pub struct Demodulator {
    constellation: Constellation,
    resampler: Resampler,
    samp_rate: usize,
    baud_rate: usize,
    carrier: usize,
    time: usize
}

impl Demodulator {
    pub fn new(n: usize, baud_rate: usize, samp_rate: usize) -> Demodulator {
        let sps = samp_rate as f32 / baud_rate as f32;
        let taps = Filter::rrc(1.0, sps as f64, 0.22, 20);

        Demodulator {
            constellation: Constellation::new(n),
            resampler: Resampler::new(sps, taps, 32),
            samp_rate: samp_rate,
            baud_rate: baud_rate,
            carrier: 1500,
            time: 0
        }
    }

    pub fn demodulate(&mut self, data: &str) {

    }
}
