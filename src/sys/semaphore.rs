extern crate libc;
use self::libc::c_void;

use sys::common::{VkStructureType, VkResult, VkAllocationCallbacks};
use sys::device::VkDevice;

pub type VkSemaphore = usize;

#[repr(C)]
pub struct VkSemaphoreCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(u32)]
pub enum VkSemaphoreCreateFlags {
    Reserved = 0,
}

#[link(name="vulkan")]
extern {
    pub fn vkCreateSemaphore(device: VkDevice, create_info: *const VkSemaphoreCreateInfo, p_allocator: *const VkAllocationCallbacks, p_semaphore: *mut VkSemaphore) -> VkResult;
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, p_allocator: *const VkAllocationCallbacks);
}
