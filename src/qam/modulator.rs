use std::f32::consts::PI;
use std::io::stderr;
use std::io::Write;
use num::Complex;

use qam::Constellation;
use fir::Filter;

pub struct Modulator {
    constellation: Constellation,
    filter: Filter,
    baud_rate: usize,
    samp_rate: usize,
    carrier: usize,
    phasor: f32,
    time: usize
}

impl Modulator {
    pub fn new(n: usize, baud_rate: usize, samp_rate: usize) -> Modulator {
        let filter = Filter::rrc(samp_rate/baud_rate, 0.22);

        Modulator {
            constellation: Constellation::new(n),
            filter: filter,
            samp_rate: samp_rate,
            baud_rate: baud_rate,
            carrier: 1500,
            phasor: 0.0,
            time: 0
        }
    }

    pub fn modulate_symbol(&mut self, sym: usize) {
        let mut out = stderr();

        let point = self.constellation.points[sym];
        let samples = self.samp_rate/self.baud_rate;

        for _ in 0..samples {
            let value = point;

            let isample = (value.re * 32767.0) as i16;
            let qsample = (value.im * 32767.0) as i16;

            out.write_all(&[(isample >> 8) as u8, (isample & 0xff) as u8]);
            out.write_all(&[(qsample >> 8) as u8, (qsample & 0xff) as u8]);

            point.scale(1.0/self.constellation.max_amplitude);

            let value = self.filter.process(point).scale(0.5);

            let isample = (value.re * 32767.0) as i16;
            let qsample = (value.im * 32767.0) as i16;

            out.write_all(&[(isample >> 8) as u8, (isample & 0xff) as u8]);
            out.write_all(&[(qsample >> 8) as u8, (qsample & 0xff) as u8]);
        }

        self.phasor += point.arg();
        self.phasor %= 2.0*PI;
    }

    pub fn modulate(&mut self, data: &str) {
        let bits = self.constellation.bits_per_symbol;
        let mut size: usize = 0;
        let mut symbol: usize = 0;

        for byte in data.bytes() {
            for i in 0..8 {
                symbol |= (((byte as usize) & (1 << i)) >> i) << size;
                size += 1;

                if size >= bits {
                    self.modulate_symbol(symbol);
                    symbol = 0;
                    size = 0;
                }
            }
        }

        if size <= bits {
            self.modulate_symbol(symbol);
        }
    }
}
