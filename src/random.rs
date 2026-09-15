use std::time::{SystemTime, UNIX_EPOCH};


//Xosiro128++
pub struct RandomGenerator {
    state: u64
}

impl RandomGenerator {

    pub fn new(seed_xor: u64) -> RandomGenerator {
        //initializet with system time and xored with input seed
        let seed: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) =>  duration.as_millis() as u64,
            Err(_) => 0x1337CAFE1057BABE,
        };

        RandomGenerator {
            state: seed ^ seed_xor
        }
    }

    pub fn next_number(&mut self) -> u64 {
        let mut z = self.state + 0x9e3779b97f4a7c15;
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
        z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
        self.state = z ^ (z >> 31);
        self.state
    }
}
