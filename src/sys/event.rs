extern crate libc;
use self::libc::c_void;

use sys::common::{VkStructureType, VkResult, VkAllocationCallbacks};
use sys::device::VkDevice;

pub type VkEvent = usize;

#[repr(C)]
pub struct VkEventCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkEventCreateFlags,
}

#[repr(u32)]
pub enum VkEventCreateFlags {
    Reserved = 0,
}

#[link(name="vulkan")]
extern {
    pub fn vkCreateEvent(device: VkDevice, create_info: *const VkEventCreateInfo, p_allocator: *const VkAllocationCallbacks, p_event: *mut VkEvent) -> VkResult;
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, p_allocator: *const VkAllocationCallbacks);
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
}
