extern crate libc;
use self::libc::{c_void, uint32_t, uint64_t};

use sys::common::{VkFlags, VkBool32, VkStructureType, VkResult, VkAllocationCallbacks};
use sys::device::VkDevice;

pub type VkFence = usize;

#[repr(C)]
pub struct VkFenceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFenceCreateFlags,
}

bitflags! {
    #[repr(C)]
    pub flags VkFenceCreateFlags: VkFlags {
        const VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
    }
}

#[link(name="vulkan")]
extern {
    pub fn vkCreateFence(device: VkDevice, create_info: *const VkFenceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_fence: *mut VkFence) -> VkResult;
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, p_allocator: *const VkAllocationCallbacks);
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
    pub fn vkResetFences(device: VkDevice, fence_count: uint32_t, p_fences: *const VkFence) -> VkResult;
    pub fn vkWaitForFences(device: VkDevice, fence_count: uint32_t, p_fences: *const VkFence, wait_all: VkBool32, timeout: uint64_t) -> VkResult;
}
