extern crate vkrs;

use vkrs::instance::Instance;
use vkrs::device::{QueuePriority, Device};
use vkrs::debug::{DebugReportCallbackEXT, DebugReportFlagsEXT, stderr_printer};
use std::collections::HashMap;

fn physical_device() {
    let physical_devices = {
        let instance = Instance::new(None, None).unwrap();
        instance.devices()
        //~^ Error `instance` does not live long enough
    };
}

fn device() {
    let device = {
        let instance = Instance::new(None, None).unwrap();
        let physical_devices = instance.devices().unwrap();
        //~^ Error `instance` does not live long enough
        let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<u32, Vec<QueuePriority>>>();
        Device::new(&physical_devices[0], priorities)
    };
}

fn debug() {
    let debug = {
        // Flags don't matter, runtime issue (for now)
        let instance = Instance::new(None, None).unwrap();
         DebugReportCallbackEXT::new(&instance, stderr_printer, DebugReportFlagsEXT::all())
        //~^ Error `instance` does not live long enough
    };
}

fn main() {}
