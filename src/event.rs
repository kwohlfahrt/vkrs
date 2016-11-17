use sys::event::*;
use device::Device;
use sys::common::{VkResult, VkStructureType, VK_NULL_HANDLE};
use std::ptr;

pub struct Event<'a> {
    handle: VkEvent,
    device: &'a Device<'a>,
}

impl<'a> Event<'a> {
    pub fn new(device: &'a Device) -> Result<Self, VkResult> {
        let create_info = VkEventCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_EVENT_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkEventCreateFlags::Reserved,
        };
        let mut semaphore = VK_NULL_HANDLE;
        match unsafe{vkCreateEvent(*device.handle(), &create_info, ptr::null(), &mut semaphore)} {
            VkResult::VK_SUCCESS => Ok(Event{handle: semaphore, device: device}),
            x => Err(x),
        }
    }

    pub fn signaled(&self) -> Result<bool, VkResult> {
        match unsafe {vkGetEventStatus(*self.device.handle(), self.handle)} {
            VkResult::VK_EVENT_SET => Ok(true),
            VkResult::VK_EVENT_RESET => Ok(false),
            x => Err(x)
        }
    }

    // Use &mut here, because it must be externally synchronized
    pub fn set(&mut self) -> Result<(), VkResult> {
        match unsafe {vkSetEvent(*self.device.handle(), self.handle)}{
            VkResult::VK_SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn reset(&mut self) -> Result<(), VkResult> {
        match unsafe {vkResetEvent(*self.device.handle(), self.handle)}{
            VkResult::VK_SUCCESS => Ok(()),
            x => Err(x)
        }
    }
}

impl<'a> Drop for Event<'a> {
    fn drop(&mut self) {
        unsafe{vkDestroyEvent(*self.device.handle(), self.handle, ptr::null())}
    }
}

#[cfg(test)]
mod test {
    use instance::debug_instance;
    use debug::debug_monitor;
    use std::sync::atomic::Ordering;

    use device::{Device, QueuePriority};
    use std::collections::HashMap;

    use event::*;

    #[test]
    fn create_event() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        Event::new(&device).unwrap();

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn test_event() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let event = Event::new(&device).unwrap();
        assert!(!event.signaled().unwrap());

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn set_event() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let mut event = Event::new(&device).unwrap();
        assert!(!event.signaled().unwrap());
        event.set().unwrap();
        assert!(event.signaled().unwrap());

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }

    #[test]
    fn reset_event() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance, true);
        let device = {
            let priorities = vec!((0, vec!(QueuePriority::from_float_clamped(1.0)))).into_iter().collect::<HashMap<_, _>>();
            Device::new(&instance.devices().unwrap()[0], priorities).unwrap()
        };
        let mut event = Event::new(&device).unwrap();
        assert!(!event.signaled().unwrap());
        event.set().unwrap();
        assert!(event.signaled().unwrap());
        event.reset().unwrap();
        assert!(!event.signaled().unwrap());

        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed));
    }
}
