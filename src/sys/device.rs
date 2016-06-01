extern crate libc;
use self::libc::{c_void, uint32_t, c_float, c_char};

use sys::common::{VkBool32, VkStructureType, VkResult, VkAllocationCallbacks};
use sys::instance::{VkPhysicalDevice};

pub type VkDevice = usize;
pub type VkQueue = usize;

#[repr(C)]
#[derive(Default)]
pub struct VkPhysicalDeviceFeatures {
    pub robust_buffer_access: VkBool32,
    pub full_draw_index_uint32: VkBool32,
    pub image_cube_array: VkBool32,
    pub independent_blend: VkBool32,
    pub geometry_shader: VkBool32,
    pub tessellation_shader: VkBool32,
    pub sample_rate_shading: VkBool32,
    pub dual_src_blend: VkBool32,
    pub logic_op: VkBool32,
    pub multi_draw_indirect: VkBool32,
    pub draw_indirect_first_instance: VkBool32,
    pub depth_clamp: VkBool32,
    pub depth_bias_clamp: VkBool32,
    pub fill_mode_non_solid: VkBool32,
    pub depth_bounds: VkBool32,
    pub wide_lines: VkBool32,
    pub large_points: VkBool32,
    pub alpha_to_one: VkBool32,
    pub multi_viewport: VkBool32,
    pub sampler_anisotropy: VkBool32,
    pub texture_compression_etc2: VkBool32,
    pub texture_compression_astc_ldr: VkBool32,
    pub texture_compression_bc: VkBool32,
    pub occlusion_query_precise: VkBool32,
    pub pipeline_statistics_query: VkBool32,
    pub vertex_pipeline_stores_and_atomics: VkBool32,
    pub fragment_stores_and_atomics: VkBool32,
    pub shader_tessellation_and_geometry_point_size: VkBool32,
    pub shader_image_gather_extended: VkBool32,
    pub shader_storage_image_extended_formats: VkBool32,
    pub shader_storage_image_multisample: VkBool32,
    pub shader_storage_image_read_without_format: VkBool32,
    pub shader_storage_image_write_without_format: VkBool32,
    pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,
    pub shader_sampled_image_array_dynamic_indexing: VkBool32,
    pub shader_storage_buffer_array_dynamic_indexing: VkBool32,
    pub shader_storage_image_array_dynamic_indexing: VkBool32,
    pub shader_clip_distance: VkBool32,
    pub shader_cull_distance: VkBool32,
    pub shader_float64: VkBool32,
    pub shader_int64: VkBool32,
    pub shader_int16: VkBool32,
    pub shader_resource_residency: VkBool32,
    pub shader_resource_min_lod: VkBool32,
    pub sparse_binding: VkBool32,
    pub sparse_residency_buffer: VkBool32,
    pub sparse_residency_image2_d: VkBool32,
    pub sparse_residency_image3_d: VkBool32,
    pub sparse_residency2_samples: VkBool32,
    pub sparse_residency4_samples: VkBool32,
    pub sparse_residency8_samples: VkBool32,
    pub sparse_residency16_samples: VkBool32,
    pub sparse_residency_aliased: VkBool32,
    pub variable_multisample_rate: VkBool32,
    pub inherited_queries: VkBool32,
}

#[repr(u32)]
pub enum VkDeviceQueueCreateFlags {
    Reserved = 0,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queue_family_index: uint32_t,
    pub queue_count: uint32_t,
    pub p_queue_priorities: *const c_float,
}

#[repr(u32)]
pub enum VkDeviceCreateFlags {
    Reserved = 0,
}

#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queue_create_info_count: uint32_t,
    pub p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    pub enabled_layer_count: uint32_t,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: uint32_t,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const VkPhysicalDeviceFeatures,
}

#[link(name="vulkan")]
extern {
    pub fn vkCreateDevice(physical_device: VkPhysicalDevice, create_info: *const VkDeviceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_device: *mut VkDevice) -> VkResult;
    pub fn vkDestroyDevice(device: VkDevice, p_allocator: *const VkAllocationCallbacks);
    pub fn vkGetDeviceQueue(device: VkDevice, queue_family_index: uint32_t, queue_index: uint32_t, p_queue: *mut VkQueue);
}
