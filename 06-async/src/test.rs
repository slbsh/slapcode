use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

#[test]
fn test() {
    let mut rng = SmallRng::from_entropy();
}
