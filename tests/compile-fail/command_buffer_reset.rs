extern crate vkrs;

use vkrs::instance::Instance;
use vkrs::device::{QueuePriority, Device};
use std::collections::HashMap;

fn command_buffer_reset() {
    use vkrs::command_pool::{UnifiedCommandPool, CommandPool};
    use vkrs::command_buffer::{PrimaryCommandBuffer, CommandBuffer, ResetableCommandBuffer, CommandBufferResetFlags};

    let instance = Instance::new(None, None).unwrap();
    let device = {
        let physical_devices = instance.devices().unwrap();
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
        Device::new(&physical_devices[0], priorities).unwrap()
    };
    let mut cmd_pool = UnifiedCommandPool::new(&device, 0, false).unwrap();
    let cmd_buf = &mut PrimaryCommandBuffer::allocate(&cmd_pool, 1).unwrap()[0];
    cmd_buf.reset(CommandBufferResetFlags::empty()).unwrap();
    //~^ Error no method named `reset` found
}

fn main() {}
