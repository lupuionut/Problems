// 2126. Destroying Asteroids
// ---------------------------

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut asteroids = asteroids;
        let mut state: u64 = mass as u64;
        asteroids.sort();

        for i in 0..asteroids.len() {
            if asteroids[i] as u64 > state {
                return false;
            }
            state += asteroids[i] as u64;
        }

        true
    }
}
