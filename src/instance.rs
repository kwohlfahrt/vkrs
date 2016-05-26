extern crate libc;
use self::libc::{c_uchar, uint32_t, c_void};
use std::ptr;
use common::{VkStructureType, VkResult, VkAllocationCallbacks, VK_NULL_HANDLE};

pub struct Instance {
    instance: VkInstance,
}

impl Instance {
    pub fn new<'a, L>(layers: L, extensions: L) -> Result<Instance, VkResult>
        where L: IntoIterator<Item=&'a str>
    {
        let layers = layers.into_iter().map(str::as_ptr).collect::<Vec<_>>();
        let extensions = extensions.into_iter().map(str::as_ptr).collect::<Vec<_>>();

        let create_info = VkInstanceCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkInstanceCreateFlags::Reserved,
            p_application_info: ptr::null(),
            enabled_layer_count: layers.len() as u32,
            pp_enabled_layer_names: layers.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
        };

        let mut instance = VK_NULL_HANDLE;
        unsafe {
            match vkCreateInstance(&create_info, ptr::null(), &mut instance) {
                VkResult::VK_SUCCESS => Ok(Instance{instance: instance}),
                x => Err(x),
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
    p_application_name: *const c_uchar,
    application_version: uint32_t,
    p_engine_name: *const c_uchar,
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
    pp_enabled_layer_names: *const *const c_uchar,
    enabled_extension_count: uint32_t,
    pp_enabled_extension_names: *const *const c_uchar,
}

type VkInstance = usize;

#[link(name="vulkan")]
extern {
    fn vkCreateInstance(create_info: *const VkInstanceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_instance: *mut VkInstance) -> VkResult;
    fn vkDestroyInstance(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);
}
