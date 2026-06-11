// Ultracomb
// Copyright (C) 2026 M. M. Trinidad
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see https://www.gnu.org/licenses/.

use crate::audio::biquad_filter;

// Elliptic filter of 16th order at fs/4, more details at filter-design/ellip.py
#[derive(Clone,Debug, Default)]
pub struct EllipFs4{
    cascade: biquad_filter::BiquadCascade,
}

impl EllipFs4{
    pub fn process(&mut self, sample: f32) -> f32 {
        self.cascade.process(sample)
    }

    pub fn initialize(&mut self){
        self.cascade.initialize(8);
        self.cascade.coeffs(0, 0.12121659056127686, 0.16532161816014612, 0.12121659056127683, -0.6961060631100692, 0.3755868484538472);
        self.cascade.coeffs(1, 1.0, 0.24620248255690544, 1.0, -0.1626963397493248, 0.8509834133700669);
        self.cascade.coeffs(2, 1.0, 0.04212330029660782, 0.9999999999999997, -0.023202417588582516, 0.9754547785876184);
        self.cascade.coeffs(3, 1.0, 0.011215327461125758, 0.9999999999999998, -0.0004342078437612204, 0.9967492670940632);
    }
}
