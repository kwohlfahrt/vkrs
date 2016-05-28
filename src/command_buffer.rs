use sys::common::{VkResult, VkStructureType};
use command_pool::CommandPool;
use sys::command_buffer::*;
use std::ptr;

pub type CommandBufferResetFlags = VkCommandBufferResetFlags;

pub struct PrimaryCommandBuffer<'a> {
    handle: VkCommandBuffer,
    pool: &'a CommandPool<'a>,
}

impl<'a> PrimaryCommandBuffer<'a> {
    pub fn allocate(pool: &'a CommandPool<'a>, n: u32) -> Result<Vec<Self>, VkResult> {
        let allocate_info = VkCommandBufferAllocateInfo{
            s_type: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: *pool.handle(),
            level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            command_buffer_count: n,
        };

        let device = pool.device().handle();
        let mut buffers = Vec::<VkCommandBuffer>::with_capacity(n as usize);
        unsafe {
            match vkAllocateCommandBuffers(*device, &allocate_info, buffers.as_mut_ptr()){
                VkResult::VK_SUCCESS => {buffers.set_len(n as usize)},
                x => return Err(x)
            }
        }

        Ok(buffers.into_iter().map(|buf| {
            PrimaryCommandBuffer{handle: buf, pool: pool}
        }).collect())
    }

    pub fn reset(&self, flags: CommandBufferResetFlags) -> VkResult{
        unsafe {
            vkResetCommandBuffer(self.handle, flags)
        }
    }

    pub fn handle(&self) -> &VkCommandBuffer {&self.handle}
}

impl<'a> Drop for PrimaryCommandBuffer<'a> {
    // TODO: Add allocation group, so multiple can be deallocated together
    fn drop(&mut self) {
        unsafe {
            vkFreeCommandBuffers(*self.pool.device().handle(), *self.pool.handle(), 1, &self.handle)
        }
    }
}

#[cfg(test)]
mod test {
    use instance::Instance;
    use device::{Device, QueuePriority};
    use std::collections::HashMap;
    use command_pool::{CommandPool, CommandPoolCreateFlags};
    use command_buffer::*;

    #[test]
    fn allocate_command_buffer() {
        let instance = Instance::new(None, None).unwrap();
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
        let device = Device::new(&instance.devices().unwrap()[0], priorities).unwrap();
        let cmd_pool = CommandPool::new(&device, 0, CommandPoolCreateFlags::empty()).unwrap();
        assert!(PrimaryCommandBuffer::allocate(&cmd_pool, 1).unwrap().len() > 0);
    }

    #[test]
    fn reset_command_buffer() {
        let instance = Instance::new(None, None).unwrap();
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
        let device = Device::new(&instance.devices().unwrap()[0], priorities).unwrap();
        let cmd_pool = CommandPool::new(&device, 0, CommandPoolCreateFlags::empty()).unwrap();
        let ref buf = PrimaryCommandBuffer::allocate(&cmd_pool, 1).unwrap()[0];
        buf.reset(CommandBufferResetFlags::empty());
    }
}
