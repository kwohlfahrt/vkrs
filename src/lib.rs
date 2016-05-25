mod common;
mod instance;

pub use instance::Instance;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_instance() {
        assert!(Instance::new().is_some());
    }
}
