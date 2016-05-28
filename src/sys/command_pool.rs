extern crate libc;
use self::libc::{uint32_t, c_void};
use sys::common::{VkFlags, VkStructureType, VkAllocationCallbacks, VkResult};
use sys::device::VkDevice;

pub type VkCommandPool = usize;

bitflags! {
    #[repr(C)]
    pub flags VkCommandPoolCreateFlags: VkFlags {
        const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
        const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
    }
}

bitflags! {
    #[repr(C)]
    pub flags VkCommandPoolResetFlags: VkFlags {
        const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
    }
}

#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queue_family_index: uint32_t,
}

#[link(name="vulkan")]
extern {
    pub fn vkCreateCommandPool(device: VkDevice, create_info: *const VkCommandPoolCreateInfo, p_allocator: *const VkAllocationCallbacks, p_command_pool: *mut VkCommandPool) -> VkResult;
    pub fn vkDestroyCommandPool(device: VkDevice, command_pool: VkCommandPool, p_allocator: *const VkAllocationCallbacks);
    pub fn vkResetCommandPool(device: VkDevice, command_pool: VkCommandPool, flags: VkCommandPoolResetFlags);
}
