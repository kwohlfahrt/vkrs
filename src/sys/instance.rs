extern crate libc;
use self::libc::{uint32_t, c_char, c_void};

use sys::common::{VkStructureType, VkResult, VkAllocationCallbacks};

pub type VkInstance = usize;

#[link(name="vulkan")]
extern {
    pub fn vkCreateInstance(create_info: *const VkInstanceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_instance: *mut VkInstance) -> VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);
}

#[repr(C)]
pub enum VkInstanceCreateFlags {
    Reserved = 0,
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: uint32_t,
    pub p_engine_name: *const c_char,
    pub engine_version: uint32_t,
    pub api_version: uint32_t,
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: uint32_t,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: uint32_t,
    pub pp_enabled_extension_names: *const *const c_char,
}

pub type VkPhysicalDevice = usize;

pub type PFNvkVoidFunction = extern fn() -> c_void;

#[link(name="vulkan")]
extern {
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, p_physical_device_count: *mut uint32_t, p_physical_devices: *mut VkPhysicalDevice) -> VkResult;
    pub fn vkGetInstanceProcAddr(instance: VkInstance, p_name: *const c_char) -> Option<PFNvkVoidFunction>;
}
