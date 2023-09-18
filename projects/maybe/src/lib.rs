#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// marker-start enum_type_maybe
pub enum Maybe<T> {
    Just(T),
    Nothing,
}
// marker-end enum_type_maybe

#[cfg(test)]
mod test {
    use std::{io::ErrorKind, sync::Arc, time::Duration};

    use to_byte_slice::print_type_info;

    use super::*;

    #[test]
    fn maybe_type_iflet() {
        let it: Maybe<Vec<Duration>> = Maybe::Nothing;
        // marker-start maybe_type_iflet
        if let Maybe::Just(data) = it {
            println!("got some data: {data:?}");
        } else {
            println!("there's nothing in it.");
        }
        // marker-end maybe_type_iflet
    }

    #[test]
    fn match_on_maybe() {
        let it = Maybe::Just("data!");
        // marker-start enum_type_maybe_match
        match it {
            Maybe::Just("something") => println!("found \"something\"!"),
            Maybe::Just("data!") => println!("found \"data!\""),
            Maybe::Just(something_else) => println!("found {something_else}!"),
            Maybe::Nothing => println!("there was nothing in it..."),
        }
        // marker-end enum_type_maybe_match
    }

    #[test]
    fn maybe_type_meta() {
        // marker-start maybe_type_enum_sizes
        print_type_info(&Maybe::Just(1u8));
        print_type_info(&Maybe::<u16>::Nothing);
        print_type_info(&Maybe::Just(false));
        // marker-end maybe_type_enum_sizes
    }

    #[test]
    fn maybe_type_references() {
        // marker-start maybe_type_size_of_references
        print_type_info(&Maybe::Just(&1u8));
        print_type_info(&Maybe::Just(&[2u64]));
        print_type_info(&Maybe::<&str>::Nothing);
        // marker-end maybe_type_size_of_references
    }

    #[test]
    fn maybe_type_box() {
        // marker-start maybe_type_size_of_references
        print_type_info(&Maybe::Just(Box::new(8)));
        print_type_info(&Maybe::Just(Arc::new(ErrorKind::AddrInUse)));
        print_type_info(&Maybe::<Box<String>>::Nothing);
        // marker-end maybe_type_size_of_references
    }
}