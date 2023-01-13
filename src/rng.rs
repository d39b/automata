use rand::{distributions::Uniform, prelude::Distribution};

pub struct UniformRng {
	rng: rand::rngs::ThreadRng,
}
	
impl UniformRng {
	pub fn new() -> UniformRng {
		UniformRng { rng: rand::thread_rng() }
	}

	// Returns a random element in the range from low (inclusive) to high (exclusive)
	pub fn sample<T: rand::distributions::uniform::SampleUniform>(&mut self, low: T, high: T) -> T {
        let between = Uniform::from(low..high);
		between.sample(&mut self.rng)
	}
}