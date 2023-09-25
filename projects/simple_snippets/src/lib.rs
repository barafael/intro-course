#[cfg(test)]
mod test {
    #[test]
    fn mutex() {
        use std::sync::{Mutex, MutexGuard};

        // marker-start:simple_mutex
        let value = Mutex::new(5);
        let mut guard: MutexGuard<'_, i32> = value.lock().unwrap();
        *guard = 1;
        dbg!(*guard);
        // marker-end:simple_mutex
    }

    #[test]
    fn share_to_thread() {
        // marker-start:sharing_data_with_threads
        let data: &str = "this is data!";
        let handle_1 = std::thread::spawn(move || {
            println!("data 1: {data}");
        });
        // marker-end:sharing_data_with_threads
        let handle_2 = std::thread::spawn(move || {
            println!("data 2: {data}");
        });
        handle_1.join().unwrap();
        handle_2.join().unwrap();
    }
}
