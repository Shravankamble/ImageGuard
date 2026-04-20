pub mod cache {
    use std::time::Duration;

    use moka::future::{Cache, CacheBuilder};

    #[derive(Debug, Clone)]
    pub struct ImageCache {
        pub cache: Cache<String, String>,
    }

    impl ImageCache {
        pub fn _new() -> Cache<String, String> {
            Cache::new(10_000)
        }

        pub fn default() -> Cache<String, String> {
            return CacheBuilder::new(5000)
                .time_to_idle(Duration::from_mins(5))
                .eviction_listener(|k, _, cause| log::info!("image {k} removed because {cause:?}"))
                .build();
        }

        // The TTI is set in minutes
        pub fn new(capacity: u64, ttl: u64) -> Cache<String, String> {
            return CacheBuilder::new(capacity)
                .time_to_idle(Duration::from_mins(ttl))
                .eviction_listener(|k, _, cause| log::info!("image {k} removed because {cause:?}"))
                .build();
        }
    }
}
