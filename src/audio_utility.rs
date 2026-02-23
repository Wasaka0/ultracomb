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

// Returns the mixed output sample between dry and wet. A ratio of 0 
// returns the dry sample and 1.0 returns the wet.
pub fn process_linear_dry_wet(dry: f32, wet: f32, ratio: f32) -> f32 {
    let ratio = ratio.clamp(0.0, 1.0);
    (1.0 - ratio) * dry + ratio * wet 
}