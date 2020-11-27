use rand::{self, distributions::Alphanumeric, Rng};

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod test_rnd {
    use crate::rng;

    #[test]
    fn test_generate_random_string() {
        let len = 16;
        let s = rng::generate_random_string(len);
        assert_eq!(s.len(), len);
    }
}
