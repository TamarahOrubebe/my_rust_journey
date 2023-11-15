use rand::{self, Rng};

pub fn add_one(left: usize, right: usize) -> usize {
    left + right
}

pub fn random_add_one() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=10);

    println!("The raandom  {random_number} plus 1 is {}", random_number + 1);
    random_number + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn random_works() {
        let result = random_add_one();
        assert!(result > 0);
    }
}
