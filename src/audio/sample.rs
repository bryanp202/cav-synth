#[derive(Copy, Clone, Debug, Default)]
pub struct Sample {
    value: f32,
}

impl Into<f32> for Sample {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<f32> for Sample {
    fn from(value: f32) -> Self {
        Self {
            value
        }
    }
}
