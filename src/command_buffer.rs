use sys::common::{VkResult, VkStructureType};
use command_pool::CommandPool;
use sys::command_buffer::*;
use std::ptr;

pub type CommandBufferResetFlags = VkCommandBufferResetFlags;

pub struct PrimaryCommandBuffer<'a, P: CommandPool<'a> + 'a> {
    handle: VkCommandBuffer,
    pool: &'a P,
}

impl<'a, P: CommandPool<'a>> PrimaryCommandBuffer<'a, P> {
    pub fn allocate(pool: &'a P, n: u32) -> Result<Vec<Self>, VkResult> {
        let allocate_info = VkCommandBufferAllocateInfo{
            s_type: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: *pool.handle(),
            level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            command_buffer_count: n,
        };

        let device = pool.device().handle();
        let mut buffers = Vec::<VkCommandBuffer>::with_capacity(n as usize);
        match unsafe {vkAllocateCommandBuffers(*device, &allocate_info,
                                               buffers.as_mut_ptr())} {
            VkResult::VK_SUCCESS => {
                unsafe {buffers.set_len(n as usize)};
                Ok(buffers.into_iter().map(|buf| {
                    PrimaryCommandBuffer{handle: buf, pool: pool}
                }).collect())
            },
            x => Err(x)
        }
    }

    fn reset(&mut self, flags: CommandBufferResetFlags) -> Result<(), VkResult> {
        match unsafe {vkResetCommandBuffer(*self.handle(), flags)} {
            VkResult::VK_SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn handle(&self) -> &VkCommandBuffer {&self.handle}
}

impl<'a, P: CommandPool<'a>> Drop for PrimaryCommandBuffer<'a, P> {
    fn drop(&mut self) {
        unsafe {
            vkFreeCommandBuffers(*self.pool.device().handle(), *self.pool.handle(), 1, &self.handle)
        }
    }
}

#[cfg(test)]
mod test {
    use instance::debug_instance;
    use debug::debug_monitor;
    use std::sync::atomic::Ordering;

    use device::{Device, QueuePriority};
    use std::collections::HashMap;
    use command_pool::{SplitCommandPool, CommandPool};
    use command_buffer::*;

    #[test]
    fn allocate_command_buffer() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance);

        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let cmd_pool = SplitCommandPool::new(&device, 0, false).unwrap();
        assert!(PrimaryCommandBuffer::allocate(&cmd_pool, 1).unwrap().len() > 0);
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn reset_command_buffer() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let cmd_pool = SplitCommandPool::new(&device, 0, false).unwrap();
        let primary_buf = &mut PrimaryCommandBuffer::allocate(&cmd_pool, 1).unwrap()[0];
        primary_buf.reset(CommandBufferResetFlags::empty()).unwrap();
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }
}
