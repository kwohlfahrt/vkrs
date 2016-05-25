extern crate libc;
use self::libc::{c_char, uint32_t, c_void};
use std::ptr;
use common::{VkStructureType, VkResult, VkAllocationCallbacks, VK_NULL_HANDLE};

pub struct Instance {
    instance: VkInstance,
}

impl Instance {
    pub fn new() -> Option<Instance> {
        let create_info = VkInstanceCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkInstanceCreateFlags::Reserved,
            p_application_info: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
        };

        let mut instance = VK_NULL_HANDLE;
        unsafe {
            match vkCreateInstance(&create_info, ptr::null(), &mut instance) {
                VkResult::VK_SUCCESS => Some(Instance{instance: instance}),
                _ => None,
            }
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self.instance, ptr::null())
        }
    }
}

#[repr(C)]
enum VkInstanceCreateFlags {
    Reserved = 0,
}

#[repr(C)]
struct VkApplicationInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    p_application_name: *const c_char,
    application_version: uint32_t,
    p_engine_name: *const c_char,
    engine_version: uint32_t,
    api_version: uint32_t,
}

#[repr(C)]
struct VkInstanceCreateInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    flags: VkInstanceCreateFlags,
    p_application_info: *const VkApplicationInfo,
    enabled_layer_count: uint32_t,
    pp_enabled_layer_names: *const *const c_char,
    enabled_extension_count: uint32_t,
    pp_enabled_extension_names: *const *const c_char,
}

type VkInstance = usize;

#[link(name="vulkan")]
extern {
    fn vkCreateInstance(create_info: *const VkInstanceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_instance: *mut VkInstance) -> VkResult;
    fn vkDestroyInstance(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);
}
