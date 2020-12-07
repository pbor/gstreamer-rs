// This file was generated by gir (https://github.com/gtk-rs/gir @ 1c1a8d7)
// from gir-files (https://github.com/gtk-rs/gir-files @ 01ae47c9)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use glib_sys as glib;
use gstreamer_gl_sys as gst_gl;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstGLDisplayWaylandClass {
    pub object_class: gst_gl::GstGLDisplayClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayWaylandClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "GstGLDisplayWaylandClass @ {:?}",
            self as *const _
        ))
        .field("object_class", &self.object_class)
        .field("_padding", &self._padding)
        .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstGLDisplayWayland {
    pub parent: gst_gl::GstGLDisplay,
    pub display: gpointer,
    pub registry: gpointer,
    pub compositor: gpointer,
    pub subcompositor: gpointer,
    pub shell: gpointer,
    pub foreign_display: gboolean,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayWayland {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayWayland @ {:?}", self as *const _))
            .field("parent", &self.parent)
            .field("display", &self.display)
            .field("registry", &self.registry)
            .field("compositor", &self.compositor)
            .field("subcompositor", &self.subcompositor)
            .field("shell", &self.shell)
            .finish()
    }
}

#[link(name = "gstgl-1.0")]
extern "C" {

    //=========================================================================
    // GstGLDisplayWayland
    //=========================================================================
    pub fn gst_gl_display_wayland_get_type() -> GType;
    pub fn gst_gl_display_wayland_new(name: *const c_char) -> *mut GstGLDisplayWayland;
    pub fn gst_gl_display_wayland_new_with_display(display: gpointer) -> *mut GstGLDisplayWayland;

}
