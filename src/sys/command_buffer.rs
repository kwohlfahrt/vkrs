extern crate libc;
use self::libc::{uint32_t, c_void};
use sys::common::{VkFlags, VkStructureType, VkResult};
use sys::device::VkDevice;
use sys::command_pool::VkCommandPool;

pub type VkCommandBuffer = usize;

#[repr(C)]
pub enum VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub command_pool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub command_buffer_count: uint32_t,
}

bitflags! {
    #[repr(C)]
    pub flags VkCommandBufferResetFlags: VkFlags {
        const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
    }
}

#[link(name="vulkan")]
extern {
    pub fn vkAllocateCommandBuffers(device: VkDevice, p_allocate_info: *const VkCommandBufferAllocateInfo, p_command_buffers: *mut VkCommandBuffer) -> VkResult;
    pub fn vkFreeCommandBuffers(device: VkDevice, command_pool: VkCommandPool, command_buffer_count: uint32_t, p_command_buffers: *const VkCommandBuffer);
    pub fn vkResetCommandBuffer(command_buffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
}
