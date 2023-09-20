// marker-start fizzbuzz_struct
#[derive(Default)]
struct Fizzbuzz(u64);
// marker-end fizzbuzz_struct

// marker-start fizzbuzz_iterator_trait_impl_1
impl Iterator for Fizzbuzz {
    type Item = String;
    // marker-end fizzbuzz_iterator_trait_impl_1
    // marker-start fizzbuzz_iterator_trait_impl_2
    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        match (self.0 % 3 == 0, self.0 % 5 == 0) {
            (true, true) => Some("Fizzbuzz!".to_string()),
            (false, true) => Some("Fizz".to_string()),
            (true, false) => Some("Buzz".to_string()),
            _ => Some(format!("{}", self.0)),
        }
    }
    // marker-end fizzbuzz_iterator_trait_impl_2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_20() {
        // marker-start first_20_elems
        let fizzbuzz = Fizzbuzz::default();
        fizzbuzz.take(20).for_each(|elem| {
            println!("{elem}");
        })
        // marker-end first_20_elems
    }
}
