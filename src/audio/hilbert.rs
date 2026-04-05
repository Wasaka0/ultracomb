/* Signalsmith's Hilbert IIR: A single-file, dependency-free Hilbert filter.

Copyright (c) 2024 Geraint Luff / Signalsmith Audio Ltd.

Released under 0BSD: BSD Zero Clause License
*/
use num_complex::Complex32;
const ORDER: usize = 12;
const COEFFS: [Complex32; ORDER] = [Complex32::new(-0.000224352093802, 0.00543499018201),
		Complex32::new(0.0107500557815, -0.0173890685681),
		Complex32::new(-0.0456795873917, 0.0229166931429),
		Complex32::new(0.11282500582, 0.00278413661237),
		Complex32::new(-0.208067578452, -0.104628958675),
		Complex32::new(0.28717837501, 0.33619239719),
		Complex32::new(-0.254675294431, -0.683033899655),
		Complex32::new(0.0481081835026, 0.954061589374),
		Complex32::new(0.227861357867, -0.891273574569),
		Complex32::new(-0.365411839137, 0.525088317271),
		Complex32::new(0.280729061131, -0.155131206606),
		Complex32::new(-0.0935061787728, 0.00512245855404)];
const POLES: [Complex32; ORDER] = [Complex32::new(-0.00495335976478, 0.0092579876872),
		Complex32::new(-0.017859491302, 0.0273493725543),
		Complex32::new(-0.0413714373155, 0.0744756910287),
		Complex32::new(-0.0882148408885, 0.178349677457),
		Complex32::new(-0.17922965812, 0.39601340223),
		Complex32::new(-0.338261800753, 0.829229533354),
		Complex32::new(-0.557688699732, 1.61298538328),
		Complex32::new(-0.735157736148, 2.79987398682),
		Complex32::new(-0.719057381172, 4.16396166128),
		Complex32::new(-0.517871025209, 5.29724826804),
		Complex32::new(-0.280197469471, 5.99598602388),
		Complex32::new(-0.0852751354531, 6.3048492377)];
const DIRECT: f32 = 0.000262057212648;
const PASSBAND_GAIN: f32 = 2.0;

#[derive(Clone, Debug, Default)]
pub struct HilbertIIR {
	coeffs: [Complex32; ORDER],
	poles: [Complex32; ORDER],
	state: [Complex32; ORDER],
	direct: f32
}

impl HilbertIIR {
	pub fn initialize(&mut self, sample_rate: f32) {
		let freq_factor = 0.46f32.min(20000.0 / sample_rate);
		let e = std::f32::consts::E;

		self.direct = DIRECT * 2.0 * PASSBAND_GAIN * freq_factor;
		for i in 0..ORDER {
			self.coeffs[i] = COEFFS[i] * freq_factor * PASSBAND_GAIN;
			let mag = e.powf(POLES[i].re);
			self.poles[i] = Complex32::new(mag * POLES[i].im.cos(), mag * POLES[i].im.sin());
		}
	}
    pub fn process(&mut self, sample: f32) -> (f32,f32) {
		for i in 0..ORDER {
			self.state[i] = self.state[i] * self.poles[i] + sample * self.coeffs[i];
		}

		let mut result = Complex32::new(sample * self.direct,0.0);
		for i in 0..ORDER {
			result += self.state[i];
		}
		return (result.re, result.im)
	}
}