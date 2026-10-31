
/// Standard Radix-2 FFT plan
pub struct FftPlan
{
    n: usize,
    twiddles: Vec<[f32; 2]>
}

impl FftPlan {
    /// Constructor
    pub fn new(n: usize) -> Self {
        todo!();
    }

    /// Forward FFT transform
    pub fn forward(&self, x: &mut[[f32; 2]]) {
        todo!()
    }
}