use sys::fence::*;
use device::Device;
use sys::common::{VkResult, VkStructureType, VK_NULL_HANDLE, VkBool32};
use std::ptr;

pub struct Fence<'a> {
    handle: VkFence,
    device: &'a Device<'a>,
}

impl<'a> Fence<'a> {
    pub fn new(device: &'a Device, signaled: bool) -> Result<Self, VkResult> {
        let create_info = VkFenceCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: if signaled {
                VK_FENCE_CREATE_SIGNALED_BIT
            } else {
                VkFenceCreateFlags::empty()
            }
        };

        let mut fence = VK_NULL_HANDLE;
        match unsafe{vkCreateFence(*device.handle(), &create_info, ptr::null(), &mut fence)} {
            VkResult::VK_SUCCESS => Ok(Fence{handle: fence, device: device}),
            x => Err(x),
        }
    }

    pub fn signaled(&self) -> Result<bool, VkResult> {
        match unsafe {vkGetFenceStatus(*self.device.handle(), self.handle)} {
            VkResult::VK_SUCCESS => Ok(true),
            VkResult::VK_NOT_READY => Ok(false),
            x => Err(x)
        }
    }

    pub fn reset(&mut self) -> Result<(), VkResult> {
        match unsafe{vkResetFences(*self.device.handle(), 1, &self.handle)} {
            VkResult::VK_SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn wait(&self, timeout: u64) -> Result<bool, VkResult> {
        match unsafe{vkWaitForFences(*self.device.handle(), 1, &self.handle, VkBool32::True, timeout)} {
            VkResult::VK_SUCCESS => Ok(true),
            VkResult::VK_TIMEOUT => Ok(false),
            x => Err(x),
        }
    }
}

impl<'a> Drop for Fence<'a> {
    fn drop(&mut self) {
        unsafe {vkDestroyFence(*self.device.handle(), self.handle, ptr::null())}
    }
}

#[cfg(test)]
mod test {
    use instance::debug_instance;
    use debug::debug_monitor;
    use std::sync::atomic::Ordering;

    use device::{Device, QueuePriority};
    use std::collections::HashMap;

    use fence::*;

    #[test]
    fn create_fence_unsignaled() {
        let instance = debug_instance();
        // Testing un-submitted fence is a warning, so ignore them
        let (errs, dbg) = debug_monitor(&instance, false);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let fence = Fence::new(&device, false).unwrap();

        assert!(!fence.signaled().unwrap());
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn create_fence_signaled() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let fence = Fence::new(&device, true).unwrap();

        assert!(fence.signaled().unwrap());
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn reset_fence() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, false);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let mut fence = Fence::new(&device, true).unwrap();
        assert!(fence.signaled().unwrap());
        fence.reset().unwrap();
        assert!(!fence.signaled().unwrap());

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn wait_fence() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, false);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        {let fence = Fence::new(&device, true).unwrap();
         assert!(fence.wait(10_000_000).unwrap());}
        {let fence = Fence::new(&device, false).unwrap();
         assert!(!fence.wait(10_000_000).unwrap());}

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }
}
