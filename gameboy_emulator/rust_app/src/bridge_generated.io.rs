use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_load_rom(port_: i64, rom_data: *mut wire_uint_8_list) {
    wire_load_rom_impl(port_, rom_data)
}

#[no_mangle]
pub extern "C" fn wire_unload_emulator(port_: i64) {
    wire_unload_emulator_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_render_frame(port_: i64) {
    wire_render_frame_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_vblank(port_: i64) {
    wire_handle_vblank_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_set_buttons_state(port_: i64, button_states: *mut wire_uint_8_list) {
    wire_set_buttons_state_impl(port_, button_states)
}

#[no_mangle]
pub extern "C" fn wire_load(port_: i64, rom_data: *mut wire_uint_8_list) {
    wire_load_impl(port_, rom_data)
}

#[no_mangle]
pub extern "C" fn wire_unload(port_: i64) {
    wire_unload_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_render(port_: i64) {
    wire_render_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_vblank(port_: i64) {
    wire_vblank_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_set_buttons(port_: i64, button_states: *mut wire_uint_8_list) {
    wire_set_buttons_impl(port_, button_states)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
