#![doc(html_playground_url = "https://play.rust-lang.org/")]

// marker-start:simple_doctest
/// An identity function.
///
/// ````rust
/// use simple_tests::identity;
///
/// let x = 4;
/// assert_eq!(identity(&x), &4);
/// ````
// marker-end:simple_doctest
// marker-start:identity_fn
pub fn identity<T>(elem: T) -> T {
    elem
}
// marker-end:identity_fn

pub fn broken_identity<T: Default + PartialEq>(elem: T) -> T {
    if elem == T::default() {
        panic!("Identity on default element refuses to work.")
    } else {
        elem
    }
}

// marker-start:test_module
#[cfg(test)]
mod test {
    use super::*;
    // marker-end:test_module

    // marker-start:test_should_map_integers
    #[test]
    fn should_map_integers() {
        assert!(identity(3) == 3);
        assert_eq!(4, identity(4));
    }
    // marker-end:test_should_map_integers

    // marker-start:test_should_map_function_pointer
    // #[test]
    // fn should_map_function_pointer() {
    //     let id = &identity::<bool>;
    //     assert_eq!(id, identity(id));
    // }
    // marker-end:test_should_map_function_pointer

    // marker-start:test_should_panic
    #[allow(unconditional_panic)]
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn refuses_to_divide_by_zero() {
        let n = rand::random::<i8>();
        // let n  = 0;
        let _ = n / 0;
    }
    // marker-end:test_should_panic

    // marker-start:start_paused_timeout
    #[tokio::test(start_paused = true)]
    async fn should_time_out() {
        use std::time::Duration;
        use tokio::{sync::mpsc, time::timeout};
        const TIMEOUT: Duration = Duration::from_millis(500);

        let now = std::time::Instant::now();
        let (_tx, mut rx) = mpsc::channel::<String>(16);
        assert!(timeout(TIMEOUT, rx.recv()).await.is_err());
        // marker-end:start_paused_timeout
        dbg!(now.elapsed());
    }

    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        // marker-start:simple_proptest
        #[test]
        fn should_map_small_i32(a in 0..1000i32) {
            assert_eq!(a, identity(a));
        }
        // marker-end:simple_proptest

        // marker-start:any_strategy_proptest
        #[test]
        fn should_map_any_i32(a in any::<i32>()) {
            assert_eq!(a, identity(a));
        }
        // marker-end:any_strategy_proptest

        // marker-start:vec_strategy_proptest
        #[test]
        fn should_map_vec_elems(a in prop::collection::vec(any::<u128>(), 1..1000)) {
            assert!(a.into_iter().all(|elem| elem == identity(elem)));
        }
        // marker-end:vec_strategy_proptest
    }
}
