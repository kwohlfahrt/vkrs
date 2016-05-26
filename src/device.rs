extern crate libc;
use self::libc::{uint32_t};
use common::{VkResult};
use instance::{VkInstance, Instance};
use std::ptr;

pub struct PhysicalDevice {
    physical_device: VkPhysicalDevice
}

impl PhysicalDevice {
    pub fn enumerate(instance: &Instance) -> Result<Vec<Self>, VkResult> {
        let mut ndevices = 0;
        unsafe {
            match vkEnumeratePhysicalDevices(instance.instance,
                                             &mut ndevices,
                                             ptr::null_mut()) {
                VkResult::VK_SUCCESS => {}
                x => return Err(x)
            };
        }
        let mut devices = Vec::<VkPhysicalDevice>::with_capacity(ndevices as usize);
        unsafe {
            match vkEnumeratePhysicalDevices(instance.instance,
                                             &mut (devices.capacity() as u32),
                                             devices.as_mut_ptr()) {
                VkResult::VK_SUCCESS => {}
                x => return Err(x)
            };
            devices.set_len(ndevices as usize);
        }
        devices.into_iter().map(|dev| {Ok(PhysicalDevice{physical_device: dev})}).collect()
    }
}

type VkPhysicalDevice = usize;

#[link(name="vulkan")]
extern {
    fn vkEnumeratePhysicalDevices(instance: VkInstance, p_physical_device_count: *mut uint32_t, p_physical_devices: *mut VkPhysicalDevice) -> VkResult;
}
