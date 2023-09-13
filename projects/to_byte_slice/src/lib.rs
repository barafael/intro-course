use itertools::Itertools;

pub fn make_byte_slice<T: Sized>(value: &T) -> &[u8] {
    unsafe {
        let len = std::mem::size_of::<T>();
        let data = value as *const T as *const u8;
        std::slice::from_raw_parts(data, len)
    }
}

pub fn print_meta<T>(value: &T) {
    let slice = make_byte_slice(value);
    println!(
        "type name: {},\nlen: {},\nbytes: {}",
        std::any::type_name::<T>(),
        slice.len(),
        slice.iter().join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misaligned_struct() {
        // marker-start simple_misaligned_reprc_struct
        #[repr(C)]
        struct A(u8, u8, u32);
        // marker-end simple_misaligned_reprc_struct

        // marker-start print_meta
        let a = A(1, 2, 4);
        print_meta(&a);
        // marker-end print_meta
    }

    #[test]
    fn misaligned_struct_packed() {
        // marker-start simple_misaligned_reprpacked_struct
        #[repr(packed)]
        struct A(u8, u8, u32);
        // marker-end simple_misaligned_reprpacked_struct

        // marker-start print_meta_packed
        let a = A(1, 2, 4);
        print_meta(&a);
        // marker-end print_meta_packed
    }

    #[test]
    fn maybe_type() {
        // marker-start enum_type_maybe
        enum Maybe<T> {
            Just(T),
            Nothing,
        }
        // marker-end enum_type_maybe

        print_meta(&Maybe::Just(1u8));
        print_meta(&Maybe::<u16>::Nothing);
        print_meta(&Maybe::Just(&true));
    }
}
