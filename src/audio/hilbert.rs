/* Signalsmith's Hilbert IIR: A single-file, dependency-free Hilbert filter.

Copyright (c) 2024 Geraint Luff / Signalsmith Audio Ltd.

Released under 0BSD: BSD Zero Clause License
*/
// Adapted code from: https://github.com/Signalsmith-Audio/hilbert-iir
use num_complex::Complex64;
const ORDER: usize = 12;
const COEFFS: [Complex64; ORDER] = [Complex64::new(-0.000224352093802, 0.00543499018201),
		Complex64::new(0.0107500557815, -0.0173890685681),
		Complex64::new(-0.0456795873917, 0.0229166931429),
		Complex64::new(0.11282500582, 0.00278413661237),
		Complex64::new(-0.208067578452, -0.104628958675),
		Complex64::new(0.28717837501, 0.33619239719),
		Complex64::new(-0.254675294431, -0.683033899655),
		Complex64::new(0.0481081835026, 0.954061589374),
		Complex64::new(0.227861357867, -0.891273574569),
		Complex64::new(-0.365411839137, 0.525088317271),
		Complex64::new(0.280729061131, -0.155131206606),
		Complex64::new(-0.0935061787728, 0.00512245855404)];
const POLES: [Complex64; ORDER] = [Complex64::new(-0.00495335976478, 0.0092579876872),
		Complex64::new(-0.017859491302, 0.0273493725543),
		Complex64::new(-0.0413714373155, 0.0744756910287),
		Complex64::new(-0.0882148408885, 0.178349677457),
		Complex64::new(-0.17922965812, 0.39601340223),
		Complex64::new(-0.338261800753, 0.829229533354),
		Complex64::new(-0.557688699732, 1.61298538328),
		Complex64::new(-0.735157736148, 2.79987398682),
		Complex64::new(-0.719057381172, 4.16396166128),
		Complex64::new(-0.517871025209, 5.29724826804),
		Complex64::new(-0.280197469471, 5.99598602388),
		Complex64::new(-0.0852751354531, 6.3048492377)];
const DIRECT: f64 = 0.000262057212648;
const PASSBAND_GAIN: f64 = 2.0;

#[derive(Clone, Debug, Default)]
pub struct HilbertIIR {
	coeffs: [Complex64; ORDER],
	poles: [Complex64; ORDER],
	state: [Complex64; ORDER],
	direct: f64
}

impl HilbertIIR {
	pub fn initialize(&mut self, sample_rate: f32) {
		let freq_factor = 0.46f64.min(20000.0 / sample_rate as f64);
		let e = std::f64::consts::E;

		self.direct = DIRECT * PASSBAND_GAIN * freq_factor;
		for i in 0..ORDER {
			self.coeffs[i] = COEFFS[i] * freq_factor * PASSBAND_GAIN;
			let mag = e.powf(POLES[i].re);
			self.poles[i] = Complex64::new(mag * POLES[i].im.cos(), mag * POLES[i].im.sin());
		}
	}
    pub fn process(&mut self, sample: f64) -> (f64,f64) {
		for i in 0..ORDER {
			self.state[i] = self.state[i] * self.poles[i] + sample * self.coeffs[i];
		}

		let mut result = Complex64::new(sample * self.direct,0.0);
		for i in 0..ORDER {
			result += self.state[i];
		}
		return (result.re, result.im)
	}
}