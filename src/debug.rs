extern crate libc;
use self::libc::{c_char, uint64_t, int32_t, c_void, size_t};
use std::ffi::CStr;
use sys::common::{VkStructureType, VkResult, VK_NULL_HANDLE, VkBool32};
use sys::instance::{vkGetInstanceProcAddr, PFNvkVoidFunction};
use sys::debug::*;
use instance::Instance;

use std::ptr;
use std::io::{self, Write};
use std::mem::transmute;

pub type DebugReportFlagsEXT = VkDebugReportFlagsEXT;

pub struct DebugReportCallbackEXT<'a> {
    callback: VkDebugReportCallbackEXT,
    instance: &'a Instance,
    destructor: PFNvkDestroyDebugReportCallbackEXT,
}

impl<'a> DebugReportCallbackEXT<'a> {
    pub fn new(instance: &'a Instance, callback: PFNvkDebugReportCallbackEXT, flags: VkDebugReportFlagsEXT) -> Result<Self, VkResult> {
        let create_info = VkDebugReportCallbackCreateInfoEXT{
            s_type: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next: ptr::null(),
            flags: flags,
            pfn_callback: callback,
            p_user_data: ptr::null_mut(),
        };

        let create_fn : PFNvkCreateDebugReportCallbackEXT;
        let destroy_fn : PFNvkDestroyDebugReportCallbackEXT;

        unsafe {
            create_fn = transmute::<PFNvkVoidFunction, PFNvkCreateDebugReportCallbackEXT>(
                match vkGetInstanceProcAddr(*instance.handle(), "vkCreateDebugReportCallbackEXT".as_ptr()) {
                    None => return Err(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
                    Some(x) => x,
                });
            destroy_fn = transmute::<PFNvkVoidFunction, PFNvkDestroyDebugReportCallbackEXT>(
                match vkGetInstanceProcAddr(*instance.handle(), "vkDestroyDebugReportCallbackEXT".as_ptr()) {
                    None => return Err(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
                    Some(x) => x,
                });
        }

        let mut callback = VK_NULL_HANDLE as VkDebugReportCallbackEXT;
        match create_fn(*instance.handle(), &create_info, ptr::null(), &mut callback) {
            VkResult::VK_SUCCESS => Ok(DebugReportCallbackEXT{callback: callback, instance: instance, destructor: destroy_fn}),
            x => Err(x),
        }
    }
}

impl<'a> Drop for DebugReportCallbackEXT<'a> {
    fn drop(&mut self) {
        (self.destructor)(*self.instance.handle(), self.callback, ptr::null());
    }
}

#[allow(unused_variables)]
pub extern "C" fn stderr_printer(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, message_code: int32_t, p_layer_prefix: *const c_char, p_message: *const c_char, p_user_data: *mut c_void) -> VkBool32 {
    let message = unsafe{CStr::from_ptr(p_message).to_bytes_with_nul()};
    io::stderr().write(message).unwrap();
    VkBool32::False
}

#[cfg(test)]
mod tests {
    use instance::{Instance, debug_instance};
    use debug::*;

    #[test]
    fn create_debug_report() {
        let instance = debug_instance();
        assert!(DebugReportCallbackEXT::new(&instance, stderr_printer, DebugReportFlagsEXT::all()).is_ok());
    }

    #[test]
    fn create_debug_report_fail() {
        let instance = Instance::new(None, None).unwrap();
        assert!(DebugReportCallbackEXT::new(&instance, stderr_printer, DebugReportFlagsEXT::all()).is_err())
    }
}
