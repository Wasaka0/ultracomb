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

use nih_plug::debug::nih_debug_assert;

// A circular buffer that allows delayed read
#[derive(Clone, Debug, Default)]
pub struct RingBuffer{
    sample_rate: f32,
    buffer_size: usize,
    audio_buffer: Vec<f32>,
    read_index: usize,
    write_index: usize,
    last_read: f32,
    now_ratio: f32,
    prev_sample_ratio: f32
}

impl RingBuffer {
    //Resizes and resets the buffer
    pub fn resize(&mut self, sample_rate: f32, max_delay: f32) {
        nih_debug_assert!(max_delay > 0.0);
        nih_debug_assert!(sample_rate > 0.0);

        self.sample_rate = sample_rate;
        self.buffer_size = (sample_rate * max_delay).ceil() as usize;
        self.audio_buffer.resize(self.buffer_size as usize, 0.0);
        self.reset()
    }

    //Change the read index to the given delay in milliseconds
    pub fn set_delay_ms(&mut self, new_delay: f32){
        let delay_samples = new_delay * self.sample_rate * 0.001;
        self.prev_sample_ratio = delay_samples.fract();
        self.now_ratio = 1.0 - self.prev_sample_ratio;
        self.move_read_index(delay_samples.trunc() as i32);
    }
    //Calculates the read index from the desired delay in samples from the write index
    fn move_read_index(&mut self, mut delay: i32){
        if delay >= (self.buffer_size as i32) {
            delay = (self.buffer_size as i32)-1;
        }
        let mut index = (self.write_index as i32) - delay;
        if index < 0{
            index += self.buffer_size as i32;
        }
        self.read_index = index as usize;
    }

    //Writes the given sample to the buffer and returns the next sample on the read index
    pub fn process(&mut self, sample: f32) -> f32 {
        self.ingest(sample);
        self.next_sample()
    }

    /// Write a single incoming sample to the buffer
    fn ingest(&mut self, sample: f32){
        //Write incoming sample
        self.audio_buffer[self.write_index] = sample;
        self.write_index = self.advance_index(self.write_index);
    }
    
    /// Rerturn current delayed sample.
    fn next_sample(&mut self) -> f32 {
        let result = self.now_ratio * self.audio_buffer[self.read_index] + self.prev_sample_ratio * self.last_read;
        self.last_read = self.audio_buffer[self.read_index];
        self.read_index = self.advance_index(self.read_index);
        result
    }

    //Advances and returns the given index, looping around the buffer size
    fn advance_index(&self,index: usize) -> usize{
        let mut result = index + 1;
        if result == self.buffer_size {
            result = 0;
        }
        result
    }

    //Resets the state of the buffer
    pub fn reset(&mut self) {
        self.read_index = 0;
        self.write_index = 0;
        self.move_read_index(0);
        self.audio_buffer.fill(0.0);
        self.last_read = 0.0;
    }
}