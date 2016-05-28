use sys::common::{VkResult, VkStructureType, VK_NULL_HANDLE};
use device::Device;
use sys::command_pool::*;
use std::ptr;

pub type CommandPoolCreateFlags = VkCommandPoolCreateFlags;
pub type CommandPoolResetFlags = VkCommandPoolResetFlags;

pub struct CommandPool<'a> {
    handle: VkCommandPool,
    device: &'a Device<'a>
}

impl<'a> CommandPool<'a> {
    pub fn new(device: &'a Device, queue_family_index: u32, flags: VkCommandPoolCreateFlags) -> Result<Self, VkResult>{
        let create_info = VkCommandPoolCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: flags,
            queue_family_index: queue_family_index,
        };

        let mut command_pool = VK_NULL_HANDLE;
        unsafe {
            match vkCreateCommandPool(*device.handle(), &create_info,
                                      ptr::null(), &mut command_pool) {
                VkResult::VK_SUCCESS => Ok(CommandPool{handle: command_pool,
                                                       device: device}),
                x => Err(x)
            }
        }
    }

    // TODO: Ensure command buffers are invalidated when reset occurs
    pub fn reset(&self, flags: CommandPoolResetFlags) {
        unsafe {
            vkResetCommandPool(*self.device.handle(), self.handle, flags)
        }
    }

    pub fn handle(&self) -> &VkCommandPool {&self.handle}
    pub fn device(&self) -> &Device<'a> {self.device}
}

impl <'a> Drop for CommandPool<'a> {
    fn drop(&mut self) {
        unsafe {
            vkDestroyCommandPool(*self.device.handle(), self.handle, ptr::null());
        }
    }
}

#[cfg(test)]
mod test {
    use instance::Instance;
    use device::{Device, QueuePriority};
    use command_pool::*;
    use std::collections::HashMap;

    #[test]
    fn create_command_pool() {
        let instance = Instance::new(None, None).unwrap();
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        assert!(CommandPool::new(&device, 0, CommandPoolCreateFlags::empty()).is_ok());
    }

    #[test]
    fn reset_command_pool() {
        let instance = Instance::new(None, None).unwrap();
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let cmd_pool = CommandPool::new(&device, 0, CommandPoolCreateFlags::empty()).unwrap();
        cmd_pool.reset(CommandPoolResetFlags::empty());
    }
}
