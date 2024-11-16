#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.80.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_load_rom_impl(
    port_: MessagePort,
    rom_data: impl Wire2Api<Vec<u8>> + UnwindSafe,
    ram_data: impl Wire2Api<Option<Vec<u8>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "load_rom",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_rom_data = rom_data.wire2api();
            let api_ram_data = ram_data.wire2api();
            move |task_callback| Ok(load_rom(api_rom_data, api_ram_data))
        },
    )
}
fn wire_unload_emulator_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<Vec<u8>>>(
        WrapInfo {
            debug_name: "unload_emulator",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(unload_emulator()),
    )
}
fn wire_render_frame_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<Vec<u32>>>(
        WrapInfo {
            debug_name: "render_frame",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(render_frame()),
    )
}
fn wire_handle_vblank_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "handle_vblank",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(handle_vblank()),
    )
}
fn wire_set_buttons_state_impl(
    port_: MessagePort,
    button_states: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "set_buttons_state",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_button_states = button_states.wire2api();
            move |task_callback| Ok(set_buttons_state(api_button_states))
        },
    )
}
fn wire_load_impl(
    port_: MessagePort,
    rom_data: impl Wire2Api<Vec<u8>> + UnwindSafe,
    ram_data: impl Wire2Api<Option<Vec<u8>>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "load",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_rom_data = rom_data.wire2api();
            let api_ram_data = ram_data.wire2api();
            move |task_callback| Ok(load(api_rom_data, api_ram_data))
        },
    )
}
fn wire_unload_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<Vec<u8>>>(
        WrapInfo {
            debug_name: "unload",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(unload()),
    )
}
fn wire_render_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<Vec<u32>>>(
        WrapInfo {
            debug_name: "render",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(render()),
    )
}
fn wire_vblank_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "vblank",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(vblank()),
    )
}
fn wire_set_buttons_impl(port_: MessagePort, button_states: impl Wire2Api<Vec<u8>> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "set_buttons",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_button_states = button_states.wire2api();
            move |task_callback| Ok(set_buttons(api_button_states))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
