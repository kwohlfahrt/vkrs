extern crate libc;
use self::libc::{uint32_t, c_uchar, c_void, c_float};
use common::{VkResult, VkStructureType, VkBool32, VkAllocationCallbacks, VK_NULL_HANDLE};
use instance::{PhysicalDevice, VkPhysicalDevice};
use std::marker::PhantomData;
use std::collections::HashMap;
use std::ptr;

pub struct QueuePriority(f32);

impl QueuePriority {
    pub fn from_float(priority: f32) -> Option<QueuePriority> {
        if (0.0 <= priority) && (priority <= 1.0) {
            return Some(QueuePriority(priority))
        } else {
            None
        }
    }

    pub fn from_float_clamped(priority: f32) -> QueuePriority {
        QueuePriority(priority.min(0.0).max(1.0))
    }
}

pub struct Device<'a> {
    pub device: VkDevice,
    nqueues: HashMap<u32, u32>,
    physical_device: PhantomData<&'a PhysicalDevice<'a>>
}

impl<'a> Device<'a> {
    pub fn new(physical_device: &PhysicalDevice<'a>,
               queue_priorities: HashMap<u32, Vec<QueuePriority>>)
               -> Result<Self, VkResult> {
        let queue_create_infos = queue_priorities.iter()
            .map(|(family, priorities)| {
                VkDeviceQueueCreateInfo {
                    s_type: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: VkDeviceQueueCreateFlags::Reserved,
                    queue_family_index: *family,
                    queue_count: priorities.len() as u32,
                    p_queue_priorities: priorities.as_ptr() as *const f32,
                }})
            .collect::<Vec<_>>();
        let create_info = VkDeviceCreateInfo {
            s_type: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: VkDeviceCreateFlags::Reserved,
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
            p_enabled_features: &VkPhysicalDeviceFeatures::default(),
        };

        let nqueues = queue_priorities.iter()
            .map(|(family, priorities)| {(*family, priorities.len() as u32)})
            .collect::<HashMap<_, _>>();

        let mut device = VK_NULL_HANDLE;
        unsafe {
            match vkCreateDevice(physical_device.physical_device,
                                 &create_info, ptr::null(), &mut device) {
                VkResult::VK_SUCCESS => Ok(Device{device: device,
                                                  physical_device: PhantomData,
                                                  nqueues: nqueues}),
                x => Err(x)
            }
        }
    }

    pub fn get_queue(&self, family: u32, index: u32) -> Option<Queue<'a>> {
        match self.nqueues.get(&family) {
            None => None,
            Some(nqueues) if index < *nqueues => {
                let mut queue = VK_NULL_HANDLE;
                unsafe {vkGetDeviceQueue(self.device, family, index, &mut queue);}
                Some(Queue{queue: queue, device: PhantomData})
            }
            Some(_) => None
        }
    }
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self.device, ptr::null())
        }
    }
}

pub struct Queue<'a> {
    queue: VkQueue,
    device: PhantomData<&'a Device<'a>>
}

pub type VkDevice = usize;
type VkQueue = usize;

#[repr(C)]
#[derive(Default)]
struct VkPhysicalDeviceFeatures {
    robust_buffer_access: VkBool32,
    full_draw_index_uint32: VkBool32,
    image_cube_array: VkBool32,
    independent_blend: VkBool32,
    geometry_shader: VkBool32,
    tessellation_shader: VkBool32,
    sample_rate_shading: VkBool32,
    dual_src_blend: VkBool32,
    logic_op: VkBool32,
    multi_draw_indirect: VkBool32,
    draw_indirect_first_instance: VkBool32,
    depth_clamp: VkBool32,
    depth_bias_clamp: VkBool32,
    fill_mode_non_solid: VkBool32,
    depth_bounds: VkBool32,
    wide_lines: VkBool32,
    large_points: VkBool32,
    alpha_to_one: VkBool32,
    multi_viewport: VkBool32,
    sampler_anisotropy: VkBool32,
    texture_compression_etc2: VkBool32,
    texture_compression_astc_ldr: VkBool32,
    texture_compression_bc: VkBool32,
    occlusion_query_precise: VkBool32,
    pipeline_statistics_query: VkBool32,
    vertex_pipeline_stores_and_atomics: VkBool32,
    fragment_stores_and_atomics: VkBool32,
    shader_tessellation_and_geometry_point_size: VkBool32,
    shader_image_gather_extended: VkBool32,
    shader_storage_image_extended_formats: VkBool32,
    shader_storage_image_multisample: VkBool32,
    shader_storage_image_read_without_format: VkBool32,
    shader_storage_image_write_without_format: VkBool32,
    shader_uniform_buffer_array_dynamic_indexing: VkBool32,
    shader_sampled_image_array_dynamic_indexing: VkBool32,
    shader_storage_buffer_array_dynamic_indexing: VkBool32,
    shader_storage_image_array_dynamic_indexing: VkBool32,
    shader_clip_distance: VkBool32,
    shader_cull_distance: VkBool32,
    shader_float64: VkBool32,
    shader_int64: VkBool32,
    shader_int16: VkBool32,
    shader_resource_residency: VkBool32,
    shader_resource_min_lod: VkBool32,
    sparse_binding: VkBool32,
    sparse_residency_buffer: VkBool32,
    sparse_residency_image2_d: VkBool32,
    sparse_residency_image3_d: VkBool32,
    sparse_residency2_samples: VkBool32,
    sparse_residency4_samples: VkBool32,
    sparse_residency8_samples: VkBool32,
    sparse_residency16_samples: VkBool32,
    sparse_residency_aliased: VkBool32,
    variable_multisample_rate: VkBool32,
    inherited_queries: VkBool32,
}

#[repr(C)]
enum VkDeviceQueueCreateFlags {
    Reserved = 0,
}

#[repr(C)]
struct VkDeviceQueueCreateInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    flags: VkDeviceQueueCreateFlags,
    queue_family_index: uint32_t,
    queue_count: uint32_t,
    p_queue_priorities: *const c_float,
}

#[repr(C)]
enum VkDeviceCreateFlags {
    Reserved = 0,
}

#[repr(C)]
struct VkDeviceCreateInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    flags: VkDeviceCreateFlags,
    queue_create_info_count: uint32_t,
    p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    enabled_layer_count: uint32_t,
    pp_enabled_layer_names: *const *const c_uchar,
    enabled_extension_count: uint32_t,
    pp_enabled_extension_names: *const *const c_uchar,
    p_enabled_features: *const VkPhysicalDeviceFeatures,
}

#[link(name="vulkan")]
extern {
    fn vkCreateDevice(physical_device: VkPhysicalDevice, create_info: *const VkDeviceCreateInfo, p_allocator: *const VkAllocationCallbacks, p_device: *mut VkDevice) -> VkResult;
    fn vkDestroyDevice(device: VkDevice, p_allocator: *const VkAllocationCallbacks);
    fn vkGetDeviceQueue(device: VkDevice, queue_family_index: uint32_t, queue_index: uint32_t, p_queue: *mut VkQueue);
}
