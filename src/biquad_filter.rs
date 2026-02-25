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

// A biquad filter that provides multiple filter types 
#[derive(Clone, Debug, Default)]
pub struct BiquadFilter{
    coefficients: BiquadCoefficients,
    samples: SampleStorage
}

// Coefficients used to process the filter output
#[derive(Clone, Debug, Default)]
struct BiquadCoefficients{
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32
}

// Samples that need to be recalled during filtering process
#[derive(Clone, Debug, Default)]
struct SampleStorage{
    x1: f32,
    x2: f32,
    y0: f32,
    y1: f32,
    y2: f32
}
impl BiquadFilter {
}