use sys::common::{VkResult, VkStructureType, VK_NULL_HANDLE};
use sys::device::*;
use instance::PhysicalDevice;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::ptr;

pub struct QueuePriority(f32);

impl QueuePriority {
    pub fn from_float(priority: f32) -> Option<QueuePriority> {
        if (0.0 <= priority) && (priority <= 1.0) {
            return Some(QueuePriority(priority))
        } else {
            None
        }
    }

    pub fn from_float_clamped(priority: f32) -> QueuePriority {
        QueuePriority(priority.min(0.0).max(1.0))
    }
}

pub struct Device<'a> {
    handle: VkDevice,
    nqueues: HashMap<u32, u32>,
    physical_device: PhantomData<&'a PhysicalDevice<'a>>
}

impl<'a> Device<'a> {
    pub fn new(physical_device: &PhysicalDevice<'a>,
               queue_priorities: HashMap<u32, Vec<QueuePriority>>)
               -> Result<Self, VkResult> {
        let queue_create_infos = queue_priorities.iter()
            .map(|(family, priorities)| {
                VkDeviceQueueCreateInfo {
                    s_type: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: VkDeviceQueueCreateFlags::Reserved,
                    queue_family_index: *family,
                    queue_count: priorities.len() as u32,
                    p_queue_priorities: priorities.as_ptr() as *const f32,
                }})
            .collect::<Vec<_>>();
        let create_info = VkDeviceCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkDeviceCreateFlags::Reserved,
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
            p_enabled_features: &VkPhysicalDeviceFeatures::default(),
        };

        let nqueues = queue_priorities.iter()
            .map(|(family, priorities)| {(*family, priorities.len() as u32)})
            .collect::<HashMap<_, _>>();

        let mut device = VK_NULL_HANDLE;
        unsafe {
            match vkCreateDevice(*physical_device.handle(),
                                 &create_info, ptr::null(), &mut device) {
                VkResult::VK_SUCCESS => Ok(Device{handle: device,
                                                  physical_device: PhantomData,
                                                  nqueues: nqueues}),
                x => Err(x)
            }
        }
    }

    pub fn get_queue(&self, family: u32, index: u32) -> Option<Queue<'a>> {
        match self.nqueues.get(&family) {
            None => None,
            Some(nqueues) if index < *nqueues => {
                let mut queue = VK_NULL_HANDLE;
                unsafe {vkGetDeviceQueue(self.handle, family, index, &mut queue);}
                Some(Queue{queue: queue, device: PhantomData})
            }
            Some(_) => None
        }
    }

    pub fn handle(&self) -> &VkDevice {&self.handle}
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self.handle, ptr::null())
        }
    }
}

pub struct Queue<'a> {
    queue: VkQueue,
    device: PhantomData<&'a Device<'a>>
}

#[cfg(test)]
mod tests {
    use instance::Instance;
    use device::*;
    use std::collections::HashMap;

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
