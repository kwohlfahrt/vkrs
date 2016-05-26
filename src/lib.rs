mod common;
mod instance;

pub use instance::Instance;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_instance() {
        assert!(Instance::new(None, None).is_ok());
    }

    #[test]
    fn create_layers() {
        let layers = vec!("VK_LAYER_LUNARG_standard_validation");
        let exts = vec!("VK_EXT_debug_report");
        assert!(Instance::new(layers, exts).is_ok());
    }
}
