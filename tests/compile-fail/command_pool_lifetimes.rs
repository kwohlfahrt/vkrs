extern crate vkrs;

use vkrs::instance::Instance;
use vkrs::device::{QueuePriority, Device};
use std::collections::HashMap;

fn command_buffer() {
    use vkrs::command_pool::{CommandPool, CommandPoolCreateFlags};
    use vkrs::command_buffer::PrimaryCommandBuffer;

    let instance = Instance::new(None, None).unwrap();
    let device = {
        let physical_devices = instance.devices().unwrap();
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
        Device::new(&physical_devices[0], priorities).unwrap()
    };
    let cmd_bufs = {
        let cmd_pool = CommandPool::new(&device, 0, false).unwrap();
        PrimaryCommandBuffer::allocate(&cmd_pool, 1).unwrap()
        //~^ Error `cmd_pool` does not live long enough
    };
}

fn main() {}
