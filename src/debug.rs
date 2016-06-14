extern crate libc;
use self::libc::{c_char, uint64_t, int32_t, c_void, size_t};
use std::ffi::{CStr, CString};
use sys::common::{VkStructureType, VkResult, VK_NULL_HANDLE, VkBool32};
use sys::instance::{vkGetInstanceProcAddr, PFNvkVoidFunction};
use sys::debug::*;
use std::panic::{catch_unwind, AssertUnwindSafe, UnwindSafe};
use instance::Instance;

use std::ptr;
use std::io::{self, Write};
use std::mem::transmute;

pub type DebugReportFlagsEXT = VkDebugReportFlagsEXT;
type PFNDebugReportCallbackEXT = Fn(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, &CStr, &CStr) -> VkBool32;

pub struct DebugReportCallbackEXT<'a, 'b> {
    handle: VkDebugReportCallbackEXT,
    instance: &'a Instance,
    destructor: PFNvkDestroyDebugReportCallbackEXT,
    message: PFNvkDebugReportMessageEXT,
    #[allow(dead_code)] // used in callback_handler
    callback: Box<Box<Fn(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, &CStr, &CStr) -> VkBool32 + 'b + Sync>>,
}

extern fn callback_handler(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, message_code: int32_t, p_layer_prefix: *const c_char, p_message: *const c_char, p_user_data: *mut c_void) -> VkBool32 {
    let closure: &Box<PFNDebugReportCallbackEXT> = unsafe {transmute(p_user_data)};
    let message = unsafe{CStr::from_ptr(p_message)};
    let layer_prefix = unsafe{CStr::from_ptr(p_layer_prefix)};
    catch_unwind(AssertUnwindSafe(|| {
        closure(flags, object_type, object, location, message_code, layer_prefix, message)
    })).unwrap_or(VkBool32::False)
}

impl<'a, 'b> DebugReportCallbackEXT<'a, 'b> {
    pub fn new<F>(instance: &'a Instance, callback: F, flags: VkDebugReportFlagsEXT) -> Result<Self, VkResult>
        where F: Fn(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, &CStr, &CStr) -> VkBool32 + 'b + Sync + UnwindSafe
    {
        // Type annotation here is necessary
        let callback : Box<Box<Fn(VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, uint64_t, size_t, int32_t, &CStr, &CStr) -> VkBool32 + 'b + Sync>>
            = Box::new(Box::new(callback));
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
        let message_fn : PFNvkDebugReportMessageEXT;
        let message_name = CString::new("vkDebugReportMessageEXT").unwrap();

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
            message_fn = transmute::<PFNvkVoidFunction, PFNvkDebugReportMessageEXT>(
                match vkGetInstanceProcAddr(*instance.handle(), message_name.as_ptr()) {
                    None => return Err(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
                    Some(x) => x,
                });
        }

        let mut handle = VK_NULL_HANDLE as VkDebugReportCallbackEXT;
        match create_fn(*instance.handle(), &create_info, ptr::null(), &mut handle) {
            VkResult::VK_SUCCESS => Ok(DebugReportCallbackEXT{handle: handle, instance: instance, destructor: destroy_fn, message: message_fn, callback: callback}),
            x => Err(x),
        }
    }

    // FIXME: This acts per-instance, not per-callback.
    // Move to Instance? Add a trait to Instance?
    pub fn message(&self, flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, object: uint64_t, location: size_t, message_code: int32_t, layer_prefix: &CStr, message: &CStr) {
        (self.message)(*self.instance.handle(), flags, object_type, object, location, message_code, layer_prefix.as_ptr(), message.as_ptr());
    }
}

impl<'a, 'b> Drop for DebugReportCallbackEXT<'a, 'b> {
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

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
pub fn debug_monitor<'a>(instance: &'a Instance) -> (Arc<AssertUnwindSafe<AtomicBool>>, DebugReportCallbackEXT<'a, 'a>) {
    use std::sync::atomic::Ordering;
    let flag = Arc::new(AssertUnwindSafe(AtomicBool::new(false)));
    let closure = {
        let flag = flag.clone();
        move |_,_,_,_,_,_:&_,_:&_| {
            flag.store(true, Ordering::Relaxed);
            VkBool32::False
        }
    };
    let flags = VkDebugReportFlagsEXT::all()
        ^ VK_DEBUG_REPORT_DEBUG_BIT_EXT
        ^ VK_DEBUG_REPORT_INFORMATION_BIT_EXT;
    (flag, DebugReportCallbackEXT::new(instance, closure, flags).unwrap())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;
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

    #[test]
    fn closure_callback() {
        use sys::common::VkBool32;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::panic::AssertUnwindSafe;

        let flag = AssertUnwindSafe(AtomicBool::new(false));
        {
            let instance = debug_instance();
            let closure = |_, _, _, _, _, _: &_, _: &_| {flag.store(true, Ordering::Relaxed); VkBool32::False};
            DebugReportCallbackEXT::new(&instance, closure, DebugReportFlagsEXT::all()).unwrap();
        }
        // Adding a callback triggers with the DEBUG level enabled triggers the callback
        assert!(flag.load(Ordering::Relaxed))
    }

    #[test]
    fn closure_message() {
        use sys::debug::{VK_DEBUG_REPORT_ERROR_BIT_EXT, VkDebugReportObjectTypeEXT};
        use sys::common::VkBool32;
        use std::ffi::CString;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::panic::AssertUnwindSafe;

        let flag = AssertUnwindSafe(AtomicBool::new(false));
        {
            let instance = debug_instance();
            let closure = |_, _, _, _, _, _: &_, _: &_| {flag.store(true, Ordering::Relaxed); VkBool32::False};
            let dbg = DebugReportCallbackEXT::new(&instance, closure, DebugReportFlagsEXT::all()).unwrap();
            dbg.message(VK_DEBUG_REPORT_ERROR_BIT_EXT, VkDebugReportObjectTypeEXT::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT, 0, 0, 0, &CString::new("").unwrap(), &CString::new("monitor").unwrap());
        }
        assert!(flag.load(Ordering::Relaxed))
    }

    #[test]
    fn debug_monitor_ok() {
        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance);
        drop(dbg);
        assert!(!errs.load(Ordering::Relaxed))
    }

    #[test]
    fn debug_monitor_message() {
        use sys::debug::{VK_DEBUG_REPORT_ERROR_BIT_EXT, VkDebugReportObjectTypeEXT};
        use std::ffi::CString;

        let instance = debug_instance();
        let (errs, dbg) = debug_monitor(&instance);
        dbg.message(VK_DEBUG_REPORT_ERROR_BIT_EXT, VkDebugReportObjectTypeEXT::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT, 0, 0, 0, &CString::new("").unwrap(), &CString::new("monitor").unwrap());
        drop(dbg);
        assert!(errs.load(Ordering::Relaxed))
    }
}
