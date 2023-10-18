#![feature(assert_matches)]

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
// marker-start:enum_type_maybe
pub enum Maybe<T> {
    Something(T),
    #[default]
    Nothing,
}
// marker-end:enum_type_maybe

impl<T> Maybe<T> {
    // marker-start:maybe_fn_once
    pub fn unwrap(self) -> T {
        match self {
            Maybe::Something(t) => t,
            Maybe::Nothing => panic!("`unwrap` on a `Maybe::Nothing`!"),
        }
    }
    // marker-end:maybe_fn_once

    // marker-start:maybe_fn
    pub fn is_something(&self) -> bool {
        //matches!(self, &Maybe::Something(_))
        match self {
            Maybe::Something(_) => true,
            Maybe::Nothing => false,
        }
    }
    // marker-end:maybe_fn

    // marker-start:maybe_fn_mut
    pub fn take(&mut self) -> Maybe<T> {
        std::mem::replace(self, Maybe::Nothing)
    }
    // marker-end:maybe_fn_mut
}

#[cfg(test)]
mod test {
    use std::{io::ErrorKind, sync::Arc, time::Duration};

    use to_byte_slice::print_type_info;

    use super::*;

    #[test]
    fn should_return_just_when_unwrapped() {
        let maybe_bool = Maybe::Something(true);
        assert!(maybe_bool.unwrap());
    }

    #[test]
    fn should_be_something() {
        let mut maybe_str = Maybe::Nothing;
        assert!(!maybe_str.is_something());
        maybe_str = Maybe::Something("hi");
        assert!(maybe_str.is_something());
        // assert!(matches!(maybe_str, Maybe::Something(_)));
        // std::assert_matches::assert_matches!(maybe_str, Maybe::Something(_));
    }

    #[test]
    fn take_out() {
        let mut maybe_not = Maybe::Something(false);
        assert_eq!(Maybe::Something(false), maybe_not.take());
        assert!(!maybe_not.is_something());
    }

    #[test]
    fn maybe_type_iflet() {
        let it: Maybe<Vec<Duration>> = Maybe::Nothing;
        // marker-start:maybe_type_iflet
        if let Maybe::Something(data) = it {
            println!("got some data: {data:?}");
        } else {
            println!("there's nothing in it.");
        }
        // marker-end:maybe_type_iflet
    }

    #[test]
    fn match_on_maybe() {
        let it = Maybe::Something("data!");
        // marker-start:enum_type_maybe_match
        match it {
            Maybe::Something("something") => println!("found \"something\"!"),
            Maybe::Something("data!") => println!("found \"data!\""),
            Maybe::Something(something_else) => println!("found {something_else}!"),
            Maybe::Nothing => println!("there was nothing in it..."),
        }
        // marker-end:enum_type_maybe_match
    }

    #[test]
    fn maybe_type_meta() {
        // marker-start:maybe_type_enum_sizes
        print_type_info(&Maybe::Something(1u8));
        print_type_info(&Maybe::<u16>::Something(5));
        print_type_info(&Maybe::<bool>::Nothing);
        // marker-end:maybe_type_enum_sizes
    }

    #[test]
    fn maybe_type_references() {
        // marker-start:maybe_type_size_of_references
        print_type_info(&Maybe::Something(&1u8));
        print_type_info(&Maybe::Something(&[2u64]));
        print_type_info(&Maybe::<&str>::Nothing);
        // marker-end:maybe_type_size_of_references
    }

    #[test]
    fn maybe_type_box() {
        // marker-start:maybe_type_size_of_boxes
        print_type_info(&Maybe::Something(Box::new(8)));
        print_type_info(&Maybe::Something(Arc::new(ErrorKind::AddrInUse)));
        print_type_info(&Maybe::<Box<String>>::Nothing);
        // marker-end:maybe_type_size_of_boxes
    }
}
