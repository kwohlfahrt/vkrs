extern crate vkrs;

use vkrs::instance::Instance;
use vkrs::device::{QueuePriority, Device};
use std::collections::HashMap;

fn command_pool() {
    use vkrs::command_pool::{SplitCommandPool, CommandPool};

    let command_pool = {
        let instance = Instance::new(None, None).unwrap();
        let device = {
            let physical_devices = instance.devices().unwrap();
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
            Device::new(&physical_devices[0], priorities).unwrap()
        };
        SplitCommandPool::new(&device, 0, false)
    };
    //~^ Error `instance` does not live long enough
    //~^^ Error `device` does not live long enough
}

fn fence() {
    use vkrs::fence::Fence;

    let fence = {
        let instance = Instance::new(None, None).unwrap();
        let device = {
            let physical_devices = instance.devices().unwrap();
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
            Device::new(&physical_devices[0], priorities).unwrap()
        };
        Fence::new(&device, false)
    };
    //~^ Error `instance` does not live long enough
    //~^^ Error `device` does not live long enough
}

fn event() {
    use vkrs::event::Event;

    let event = {
        let instance = Instance::new(None, None).unwrap();
        let device = {
            let physical_devices = instance.devices().unwrap();
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
            Device::new(&physical_devices[0], priorities).unwrap()
        };
        Event::new(&device)
    };
    //~^ Error `instance` does not live long enough
    //~^^ Error `device` does not live long enough
}

fn Semaphore() {
    use vkrs::semaphore::Semaphore;

    let Semaphore = {
        let instance = Instance::new(None, None).unwrap();
        let device = {
            let physical_devices = instance.devices().unwrap();
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
            Device::new(&physical_devices[0], priorities).unwrap()
        };
        Semaphore::new(&device)
    };
    //~^ Error `instance` does not live long enough
    //~^^ Error `device` does not live long enough
}

fn main() {}
