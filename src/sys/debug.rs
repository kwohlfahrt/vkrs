extern crate libc;
use self::libc::{uint64_t, size_t, int32_t, c_char, c_void};
use sys::common::{VkBool32, VkResult, VkAllocationCallbacks, VkStructureType, VkFlags};
use sys::instance::VkInstance;

pub type VkDebugReportCallbackEXT = u64;

pub type PFNvkDebugReportCallbackEXT = extern fn(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, *const c_char, *const c_char, *mut c_void) -> VkBool32;

pub type PFNvkCreateDebugReportCallbackEXT = extern fn(instance: VkInstance, p_create_info: *const VkDebugReportCallbackCreateInfoEXT, p_allocator: *const VkAllocationCallbacks, p_callback: *mut VkDebugReportCallbackEXT) -> VkResult;

pub type PFNvkDestroyDebugReportCallbackEXT = extern fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, p_allocator: *const VkAllocationCallbacks);

pub type PFNvkDebugReportMessageEXT = extern fn (instance: VkInstance, flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, message_code: int32_t, p_layer_prefix: *const c_char, p_message: *const c_char);

bitflags! {
    #[repr(C)]
    pub flags VkDebugReportFlagsEXT: VkFlags {
        const VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,
        const VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,
        const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,
        const VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,
        const VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010,
    }
}

#[repr(C)]
pub enum VkDebugReportObjectTypeEXT {
    VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0,
    VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1,
    VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3,
    VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4,
    VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5,
    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6,
    VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8,
    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9,
    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10,
    VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11,
    VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12,
    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13,
    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14,
    VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17,
    VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20,
    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23,
    VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24,
    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25,
    VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26,
    VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT = 28,
}

#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkDebugReportFlagsEXT,
    pub pfn_callback: PFNvkDebugReportCallbackEXT,
    pub p_user_data: *mut c_void,
}
