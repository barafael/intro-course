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

    #[test]
    fn thing() {
        // marker-start:selection_struct
        #[derive(Debug)]
        struct Selection<'a>(&'a str);
        // marker-end:selection_struct

        // marker-start:find_tv_fn
        fn find_tv<'a>(input: &'a str) -> Option<Selection<'a>> {
            input
                .split_whitespace()
                .find(|token| token.contains("television"))
                .map(Selection)
        }
        // marker-end:find_tv_fn

        let novel = String::from(
            "The sky above the port was the color of television, tuned to a dead channel.",
        );
        dbg!(find_tv(&novel));
    }

    #[test]
    fn zero_copy_deserialization() {
        use serde::{Deserialize, Serialize};
        // marker-start:serde_deserialize_with_shared_references_1
        #[derive(Debug, Deserialize, Serialize)]
        struct Item<'a, 'b> {
            name: &'a str,
            description: &'b str,
        }
        // marker-end:serde_deserialize_with_shared_references_1
        // marker-start:serde_deserialize_with_shared_references_2
        let input = String::from(
            r#"{
            "name":"toothbrush",
            "description":"used, good condition"
        }"#,
        );
        let item: Item = serde_json::from_str(&input).unwrap();
        //drop(input); // UNCOMMENT THIS
        dbg!(item);
        // marker-end:serde_deserialize_with_shared_references_2
    }

    #[test]
    fn leak_config() -> anyhow::Result<()> {
        #[derive(Debug, Default)]
        struct Config;

        impl Config {
            pub fn load_from_disk() -> anyhow::Result<Config> {
                Ok(Config::default())
            }
        }

        // marker-start:config_leak
        let config = Config::load_from_disk()?;
        let config: &'static _ = Box::leak(Box::new(config));
        dbg!(config); // config may now be handed out among threads.
                      // marker-end:config_leak
        Ok(())
    }
}
