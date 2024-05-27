use rand::Rng;

// personal implementation for noise
// we use sin waves that build up on top of eachother to generate random noise values
pub struct NoiseGen;

impl NoiseGen {
    pub fn noise(x: f32, y: f32) -> f32 {
        let mut rng = rand::thread_rng();
        let mut total: f32 = 0.0;

        let iterations = rng.gen_range(3..10);
        for _ in 0..iterations {
            // sine with phase shift
            let b = rng.gen_range(1..10) as f32;
            let c = rng.gen_range(1..10) as f32;
            let x = rng.gen_range(-100..100) as f32;
            total += f32::sin(b * (x + c));
        }

        total / (iterations as f32)
    }
}
