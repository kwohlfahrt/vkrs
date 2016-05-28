use sys::common::{VkStructureType, VkResult, VK_NULL_HANDLE};
use sys::instance::*;

use std::ptr;
use std::marker::PhantomData;

pub struct Instance {
    handle: VkInstance,
}

impl Instance {
    pub fn new<'a, L, E>(layers: L, extensions: E) -> Result<Self, VkResult>
        where L: IntoIterator<Item=&'a str>, E: IntoIterator<Item=&'a str>
    {
        let layers = layers.into_iter().map(str::as_ptr).collect::<Vec<_>>();
        let extensions = extensions.into_iter().map(str::as_ptr).collect::<Vec<_>>();

        let create_info = VkInstanceCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkInstanceCreateFlags::Reserved,
            p_application_info: ptr::null(),
            enabled_layer_count: layers.len() as u32,
            pp_enabled_layer_names: layers.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
        };

        let mut instance = VK_NULL_HANDLE;
        unsafe {
            match vkCreateInstance(&create_info, ptr::null(), &mut instance) {
                VkResult::VK_SUCCESS => Ok(Instance{handle: instance}),
                x => Err(x),
            }
        }
    }

    pub fn devices<'a>(&'a self) -> Result<Vec<PhysicalDevice<'a>>, VkResult> {
        let mut ndevices = 0;
        unsafe {
            match vkEnumeratePhysicalDevices(self.handle,
                                             &mut ndevices,
                                             ptr::null_mut()) {
                VkResult::VK_SUCCESS => {}
                x => return Err(x)
            };
        }
        let mut devices = Vec::<VkPhysicalDevice>::with_capacity(ndevices as usize);
        unsafe {
            match vkEnumeratePhysicalDevices(self.handle,
                                             &mut (devices.capacity() as u32),
                                             devices.as_mut_ptr()) {
                VkResult::VK_SUCCESS => {}
                x => return Err(x)
            };
            devices.set_len(ndevices as usize);
        }
        devices.into_iter().map(|dev| {
            Ok(PhysicalDevice{handle: dev, instance: PhantomData})
        }).collect()
    }

    pub fn handle(&self) -> &VkInstance {&self.handle}
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self.handle, ptr::null())
        }
    }
}

pub struct PhysicalDevice<'a> {
    handle: VkPhysicalDevice,
    instance: PhantomData<&'a Instance>
}

impl<'a> PhysicalDevice<'a> {
    pub fn handle(&self) -> &VkPhysicalDevice {&self.handle}
}
