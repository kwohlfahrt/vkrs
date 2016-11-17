use sys::semaphore::*;
use device::Device;
use sys::common::{VkResult, VkStructureType, VK_NULL_HANDLE};
use std::ptr;

pub struct Semaphore<'a> {
    handle: VkSemaphore,
    device: &'a Device<'a>,
}

impl<'a> Semaphore<'a> {
    pub fn new(device: &'a Device) -> Result<Self, VkResult> {
        let create_info = VkSemaphoreCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkSemaphoreCreateFlags::Reserved,
        };
        let mut semaphore = VK_NULL_HANDLE;
        match unsafe{vkCreateSemaphore(*device.handle(), &create_info, ptr::null(), &mut semaphore)} {
            VkResult::VK_SUCCESS => Ok(Semaphore{handle: semaphore, device: device}),
            x => Err(x),
        }
    }
}

impl<'a> Drop for Semaphore<'a> {
    fn drop(&mut self) {
        unsafe{vkDestroySemaphore(*self.device.handle(), self.handle, ptr::null())}
    }
}

#[cfg(test)]
mod test {
    use instance::debug_instance;
    use debug::debug_monitor;
    use std::sync::atomic::Ordering;

    use device::{Device, QueuePriority};
    use std::collections::HashMap;

    use semaphore::*;

    #[test]
    fn create_semaphore() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        Semaphore::new(&device).unwrap();

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }
}
