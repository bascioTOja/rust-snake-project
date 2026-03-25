pub struct FpsSmoother {
    fps: f32,
    smoothing: f32, // 0 < smoothing < 1
}

impl FpsSmoother {
    pub fn new(smoothing: f32) -> Self {
        Self {
            fps: 60.0,
            smoothing,
        }
    }

    pub fn update(&mut self, dt: f32) -> f32 {
        let current_fps = 1.0 / dt;
        self.fps = self.fps * self.smoothing + current_fps * (1.0 - self.smoothing);
        self.fps
    }
}
