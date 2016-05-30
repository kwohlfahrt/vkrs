extern crate libc;
use self::libc::{c_char, uint64_t, int32_t, c_void, size_t};
use std::ffi::{CStr, CString};
use sys::common::{VkStructureType, VkResult, VK_NULL_HANDLE, VkBool32};
use sys::instance::{vkGetInstanceProcAddr, PFNvkVoidFunction};
use sys::debug::*;
use instance::Instance;

use std::ptr;
use std::io::{self, Write};
use std::mem::transmute;

pub type DebugReportFlagsEXT = VkDebugReportFlagsEXT;
type PFNDebugReportCallbackEXT = FnMut(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, &CStr, &CStr) -> VkBool32;

pub struct DebugReportCallbackEXT<'a> {
    handle: VkDebugReportCallbackEXT,
    instance: &'a Instance,
    destructor: PFNvkDestroyDebugReportCallbackEXT,
    #[allow(dead_code)] // used in callback_handler
    callback: Box<Box<PFNDebugReportCallbackEXT>>,
}

extern fn callback_handler(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, message_code: int32_t, p_layer_prefix: *const c_char, p_message: *const c_char, p_user_data: *mut c_void) -> VkBool32 {
    let closure: &mut Box<PFNDebugReportCallbackEXT> = unsafe {transmute(p_user_data)};
    let message = unsafe{CStr::from_ptr(p_message)};
    let layer_prefix = unsafe{CStr::from_ptr(p_layer_prefix)};
    closure(flags, object_type, object, location, message_code, layer_prefix, message)
}

impl<'a> DebugReportCallbackEXT<'a> {
    pub fn new<F>(instance: &'a Instance, callback: F, flags: VkDebugReportFlagsEXT) -> Result<Self, VkResult>
        where F: 'static + FnMut(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, &CStr, &CStr) -> VkBool32
    {
        // Type annotation here is necessary
        let callback : Box<Box<PFNDebugReportCallbackEXT>> = Box::new(Box::new(callback));
        let create_info = VkDebugReportCallbackCreateInfoEXT{
            s_type: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next: ptr::null(),
            flags: flags,
            pfn_callback: callback_handler,
            p_user_data: &*callback as *const Box<_> as *mut c_void,
        };

        let create_fn : PFNvkCreateDebugReportCallbackEXT;
        let create_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap();
        let destroy_fn : PFNvkDestroyDebugReportCallbackEXT;
        let destroy_name = CString::new("vkDestroyDebugReportCallbackEXT").unwrap();

        unsafe {
            create_fn = transmute::<PFNvkVoidFunction, PFNvkCreateDebugReportCallbackEXT>(
                match vkGetInstanceProcAddr(*instance.handle(), create_name.as_ptr()) {
                    None => return Err(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
                    Some(x) => x,
                });
            destroy_fn = transmute::<PFNvkVoidFunction, PFNvkDestroyDebugReportCallbackEXT>(
                match vkGetInstanceProcAddr(*instance.handle(), destroy_name.as_ptr()) {
                    None => return Err(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
                    Some(x) => x,
                });
        }

        let mut handle = VK_NULL_HANDLE as VkDebugReportCallbackEXT;
        match create_fn(*instance.handle(), &create_info, ptr::null(), &mut handle) {
            VkResult::VK_SUCCESS => Ok(DebugReportCallbackEXT{handle: handle, instance: instance, destructor: destroy_fn, callback: callback}),
            x => Err(x),
        }
    }
}

impl<'a> Drop for DebugReportCallbackEXT<'a> {
    fn drop(&mut self) {
        (self.destructor)(*self.instance.handle(), self.handle, ptr::null());
    }
}

#[allow(unused_variables)]
#[allow(unused_must_use)] // Can't really deal with failure to write
pub fn stderr_printer(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, message_code: int32_t, layer_prefix: &CStr, message: &CStr) -> VkBool32 {
    io::stderr().write(message.to_bytes());
    io::stderr().write(b"\n");
    io::stderr().flush();
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
    fn missing_extension() {
        let instance = Instance::new(None, None).unwrap();
        assert!(DebugReportCallbackEXT::new(&instance, stderr_printer, DebugReportFlagsEXT::all()).is_err())
    }
}
