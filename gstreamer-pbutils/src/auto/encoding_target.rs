// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::EncodingProfile;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GstEncodingTarget")]
    pub struct EncodingTarget(Object<ffi::GstEncodingTarget>);

    match fn {
        type_ => || ffi::gst_encoding_target_get_type(),
    }
}

impl EncodingTarget {
    #[doc(alias = "gst_encoding_target_new")]
    pub fn new(
        name: &str,
        category: &str,
        description: &str,
        profiles: &[EncodingProfile],
    ) -> EncodingTarget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_encoding_target_new(
                name.to_glib_none().0,
                category.to_glib_none().0,
                description.to_glib_none().0,
                profiles.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_target_get_category")]
    #[doc(alias = "get_category")]
    pub fn category(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_encoding_target_get_category(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_encoding_target_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gst_encoding_target_get_description(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_target_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_encoding_target_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_encoding_target_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gst_encoding_target_get_path(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_encoding_target_get_profile")]
    #[doc(alias = "get_profile")]
    pub fn profile(&self, name: &str) -> Option<EncodingProfile> {
        unsafe {
            from_glib_full(ffi::gst_encoding_target_get_profile(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_target_get_profiles")]
    #[doc(alias = "get_profiles")]
    pub fn profiles(&self) -> Vec<EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_encoding_target_get_profiles(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_target_save")]
    pub fn save(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gst_encoding_target_save(self.to_glib_none().0, &mut error);
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_encoding_target_save_to_file")]
    pub fn save_to_file(&self, filepath: impl AsRef<std::path::Path>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gst_encoding_target_save_to_file(
                self.to_glib_none().0,
                filepath.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_encoding_target_load")]
    pub fn load(name: &str, category: Option<&str>) -> Result<EncodingTarget, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_encoding_target_load(
                name.to_glib_none().0,
                category.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_encoding_target_load_from_file")]
    pub fn load_from_file(
        filepath: impl AsRef<std::path::Path>,
    ) -> Result<EncodingTarget, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_encoding_target_load_from_file(
                filepath.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for EncodingTarget {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

unsafe impl Send for EncodingTarget {}
unsafe impl Sync for EncodingTarget {}
