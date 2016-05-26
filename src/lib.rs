mod common;
mod instance;
mod device;

pub use instance::{Instance, PhysicalDevice};
pub use device::Device;

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

    #[test]
    fn enumerate_devices() {
        let instance = Instance::new(None, None).unwrap();
        assert!(instance.devices().unwrap().len() > 0)
    }

    #[test]
    fn create_device() {
        // Slightly convoluted, ensures lifetimes are correct
        let instance = Instance::new(None, None).unwrap();
        let device;
        {
            let physical_devices = instance.devices().unwrap();
            device = Device::new(&physical_devices[0]);
        }
        assert!(device.is_ok());

    }
}
