pub struct Container {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl Container {
    pub fn new(length: f64, width: f64, height: f64) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    pub fn get_volume(&self) -> f64 {
        self.length * self.width * self.height
    }
}