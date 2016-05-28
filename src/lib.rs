mod instance;
mod device;
mod sys;

pub use instance::{Instance, PhysicalDevice};
pub use device::{Device, QueuePriority};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn create_instance() {
        assert!(Instance::new(None, None).is_ok());
    }

    #[test]
    fn create_layers() {
        let layers = vec!("VK_LAYER_LUNARG_standard_validation");
        assert!(Instance::new(layers, None).is_ok());
    }

    #[test]
    fn create_ext() {
        let exts = vec!("VK_EXT_debug_report");
        assert!(Instance::new(None, exts).is_ok());
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
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
            device = Device::new(&physical_devices[0], priorities);
        }
        assert!(device.is_ok());

    }

    #[test]
    fn get_queue() {
        let instance = Instance::new(None, None).unwrap();
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
        let device = Device::new(&instance.devices().unwrap()[0], priorities).unwrap();
        assert!(device.get_queue(0, 0).is_some());
    }

    #[test]
    fn get_invalid_queue() {
        let instance = Instance::new(None, None).unwrap();
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
        let device = Device::new(&instance.devices().unwrap()[0], priorities).unwrap();
        assert!(device.get_queue(0, 1).is_none());
        assert!(device.get_queue(1, 0).is_none());
    }
}
