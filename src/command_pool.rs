use sys::common::{VkResult, VkStructureType, VK_NULL_HANDLE};
use device::Device;
use sys::command_pool::*;
use std::ptr;

pub type CommandPoolCreateFlags = VkCommandPoolCreateFlags;
pub type CommandPoolResetFlags = VkCommandPoolResetFlags;

pub trait CommandPool<'a> : Sized {
    const BUFFER_RESET: bool;

    unsafe fn _new(handle: VkCommandPool, device: &'a Device<'a>) -> Self;
    fn handle(&self) -> &VkCommandPool;
    fn device(&self) -> &Device<'a>;

    fn new(device: &'a Device, queue_family_index: u32, transient: bool) -> Result<Self, VkResult> {
        let create_info = VkCommandPoolCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: if Self::BUFFER_RESET {
                VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT
            } else {
                CommandPoolCreateFlags::empty()
            } | if transient {
                VK_COMMAND_POOL_CREATE_TRANSIENT_BIT
            } else {
                CommandPoolCreateFlags::empty()
            },
            queue_family_index: queue_family_index,
        };

        let mut command_pool = VK_NULL_HANDLE;
        match unsafe {vkCreateCommandPool(*device.handle(), &create_info,
                                          ptr::null(), &mut command_pool)} {
            VkResult::VK_SUCCESS => Ok(unsafe {Self::_new(command_pool, device)}),
            x => Err(x)
        }
    }

    // Command buffers must be dropped (free'd) before resetting.
    // Would be useful to add an implicit drop somehow.
    fn reset(&mut self, flags: CommandPoolResetFlags) -> Result<(), VkResult> {
        match unsafe {vkResetCommandPool(*self.device().handle(), *self.handle(), flags)} {
            VkResult::VK_SUCCESS => Ok(()),
            x => Err(x),
        }
    }
}

pub struct SplitCommandPool<'a> {
    handle: VkCommandPool,
    device: &'a Device<'a>
}

impl<'a> CommandPool<'a> for SplitCommandPool<'a> {
    const BUFFER_RESET: bool = true;

    unsafe fn _new(handle: VkCommandPool, device: &'a Device<'a>) -> Self {
        SplitCommandPool{handle: handle, device: device}
    }
    fn handle(&self) -> &VkCommandPool {&self.handle}
    fn device(&self) -> &Device<'a> {self.device}
}

impl <'a> Drop for SplitCommandPool<'a> {
    fn drop(&mut self) {
        unsafe {vkDestroyCommandPool(*self.device.handle(), self.handle, ptr::null())}
    }
}

pub struct UnifiedCommandPool<'a> {
    handle: VkCommandPool,
    device: &'a Device<'a>
}

impl<'a> CommandPool<'a> for UnifiedCommandPool<'a> {
    const BUFFER_RESET: bool = false;

    unsafe fn _new(handle: VkCommandPool, device: &'a Device<'a>) -> Self {
        UnifiedCommandPool{handle: handle, device: device}
    }
    fn handle(&self) -> &VkCommandPool {&self.handle}
    fn device(&self) -> &Device<'a> {self.device}
}

impl <'a> Drop for UnifiedCommandPool<'a> {
    fn drop(&mut self) {
        unsafe {vkDestroyCommandPool(*self.device.handle(), self.handle, ptr::null())}
    }
}

#[cfg(test)]
mod test {
    use instance::debug_instance;
    use debug::debug_monitor;
    use std::sync::atomic::Ordering;

    use device::{Device, QueuePriority};
    use command_pool::*;
    use std::collections::HashMap;

    #[test]
    fn create_command_pool() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);

        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        assert!(SplitCommandPool::new(&device, 0, false).is_ok());
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn reset_command_pool() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);

        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let mut cmd_pool = SplitCommandPool::new(&device, 0, false).unwrap();
        cmd_pool.reset(CommandPoolResetFlags::empty()).unwrap();
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }
}
