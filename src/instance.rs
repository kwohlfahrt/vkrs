use sys::common::{VkStructureType, VkResult, VK_NULL_HANDLE};
use sys::instance::*;

use std::ptr;
use std::marker::PhantomData;
use std::ffi::CString;

pub struct Instance {
    handle: VkInstance,
}

impl Instance {
    pub fn new<'a, L, E>(layers: L, extensions: E) -> Result<Self, VkResult>
        where L: IntoIterator<Item=&'a CString>, E: IntoIterator<Item=&'a CString>
    {
        let layers = layers.into_iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
        let extensions = extensions.into_iter().map(|s| s.as_ptr()).collect::<Vec<_>>();

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
        match unsafe {vkCreateInstance(&create_info, ptr::null(), &mut instance)} {
            VkResult::VK_SUCCESS => Ok(Instance{handle: instance}),
            x => Err(x),
        }
    }

    pub fn devices(&self) -> Result<Vec<PhysicalDevice>, VkResult> {
        let mut ndevices = 0;
        match unsafe {vkEnumeratePhysicalDevices(self.handle, &mut ndevices,
                                                 ptr::null_mut())} {
            VkResult::VK_SUCCESS => {}
            x => return Err(x)
        };
        let mut devices = Vec::<VkPhysicalDevice>::with_capacity(ndevices as usize);
        match unsafe {vkEnumeratePhysicalDevices(self.handle,
                                                 &mut (devices.capacity() as u32),
                                                 devices.as_mut_ptr())} {
            VkResult::VK_SUCCESS => {
                unsafe{devices.set_len(ndevices as usize)};
                Ok(devices.into_iter().map(|dev| {
                    PhysicalDevice{handle: dev, instance: PhantomData}
                }).collect())
            }
            x => Err(x)
        }
    }

    pub fn handle(&self) -> &VkInstance {&self.handle}
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {vkDestroyInstance(self.handle, ptr::null())}
    }
}

pub struct PhysicalDevice<'a> {
    handle: VkPhysicalDevice,
    instance: PhantomData<&'a Instance>
}

impl<'a> PhysicalDevice<'a> {
    pub fn handle(&self) -> &VkPhysicalDevice {&self.handle}

    pub fn queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut nqueues = 0;
        unsafe {vkGetPhysicalDeviceQueueFamilyProperties(self.handle, &mut nqueues, ptr::null_mut())};
        let mut properties = Vec::<VkQueueFamilyProperties>::with_capacity(nqueues as usize);
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle,
                                                     &mut (properties.capacity() as u32),
                                                     properties.as_mut_ptr());
            properties.set_len(nqueues as usize);
        };
        properties
    }
}

pub fn debug_instance() -> Instance {
    let exts = vec!(CString::new("VK_EXT_debug_report").unwrap());
    let layers = vec!(CString::new("VK_LAYER_LUNARG_standard_validation").unwrap());
    Instance::new(layers.iter(), exts.iter()).unwrap()
}

#[cfg(test)]
mod tests {
    use instance::*;
    use std::ffi::CString;
    use std::sync::atomic::Ordering;

    use debug::debug_monitor;

    #[test]
    fn create_instance() {
        assert!(Instance::new(None, None).is_ok());
    }

    #[test]
    fn create_layers() {
        let layers = vec!(CString::new("VK_LAYER_LUNARG_standard_validation").unwrap());
        assert!(Instance::new(layers.iter(), None).is_ok());
    }

    #[test]
    fn create_ext() {
        let exts = vec!(CString::new("VK_EXT_debug_report").unwrap());
        assert!(Instance::new(None, exts.iter()).is_ok());
    }

    #[test]
    fn enumerate_devices() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        assert!(instance.devices().unwrap().len() > 0);
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn queue_family_properties() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = &instance.devices().unwrap()[0];
        assert!(device.queue_family_properties().len() > 0);
        assert!(device.queue_family_properties()[0].queue_count > 0);

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }
}
