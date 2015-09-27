pub type RenderCallback = FnMut(&mut[f32], usize);

pub trait AudioDriver {
    // TODO: fn set_render_callback(&mut self, callback: RenderCallback);

    fn set_is_enabled(&mut self, is_enabled: bool);
    fn is_enabled(&self) -> bool;

    // TODO: set_latency
    // TODO: latency

    fn set_sample_rate(&mut self, sample_rate: i32);
    fn sample_rate(&self) -> i32;
}
