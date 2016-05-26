extern crate libc;
use self::libc::{uint32_t, c_uchar, c_void, c_float};
use common::{VkResult, VkStructureType, VkBool32, VkAllocationCallbacks, VK_NULL_HANDLE};
use instance::{VkInstance, Instance};
use std::marker::PhantomData;
use std::ptr;

pub struct PhysicalDevice<'a> {
    physical_device: VkPhysicalDevice,
    instance: PhantomData<&'a Instance>
}

impl<'a> PhysicalDevice<'a> {
    pub fn enumerate(instance: &'a Instance) -> Result<Vec<Self>, VkResult> {
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
        devices.into_iter().map(|dev| {
            Ok(PhysicalDevice{physical_device: dev, instance: PhantomData})
        }).collect()
    }
}

type VkPhysicalDevice = usize;

#[link(name="vulkan")]
extern {
    fn vkEnumeratePhysicalDevices(instance: VkInstance, p_physical_device_count: *mut uint32_t, p_physical_devices: *mut VkPhysicalDevice) -> VkResult;
}
