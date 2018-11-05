// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Asset;
use Error;
use Timeline;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst_pbutils;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Project(Object<ffi::GESProject, ffi::GESProjectClass>): Asset;

    match fn {
        get_type => || ffi::ges_project_get_type(),
    }
}

impl Project {
    pub fn new<'a, P: Into<Option<&'a str>>>(uri: P) -> Project {
        assert_initialized_main_thread!();
        let uri = uri.into();
        let uri = uri.to_glib_none();
        unsafe {
            from_glib_full(ffi::ges_project_new(uri.0))
        }
    }
}

pub trait ProjectExt {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> bool;

    fn add_encoding_profile<P: IsA<gst_pbutils::EncodingProfile>>(&self, profile: &P) -> bool;

    fn create_asset<'a, P: Into<Option<&'a str>>>(&self, id: P, extractable_type: glib::types::Type) -> bool;

    fn create_asset_sync<'a, P: Into<Option<&'a str>>>(&self, id: P, extractable_type: glib::types::Type) -> Result<Option<Asset>, Error>;

    fn get_asset(&self, id: &str, extractable_type: glib::types::Type) -> Option<Asset>;

    fn get_loading_assets(&self) -> Vec<Asset>;

    fn get_uri(&self) -> Option<String>;

    fn list_assets(&self, filter: glib::types::Type) -> Vec<Asset>;

    fn list_encoding_profiles(&self) -> Vec<gst_pbutils::EncodingProfile>;

    fn load(&self, timeline: &Timeline) -> Result<(), Error>;

    fn remove_asset<P: IsA<Asset>>(&self, asset: &P) -> bool;

    fn save<'a, P: IsA<Asset> + 'a, Q: Into<Option<&'a P>>>(&self, timeline: &Timeline, uri: &str, formatter_asset: Q, overwrite: bool) -> Result<(), Error>;

    fn connect_asset_added<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_asset_loading<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_asset_removed<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_error_loading_asset<F: Fn(&Self, &Error, &str, glib::types::Type) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_loaded<F: Fn(&Self, &Timeline) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_missing_uri<F: Fn(&Self, &Error, &Asset) -> Option<String> + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Project> + IsA<glib::object::Object>> ProjectExt for O {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> bool {
        unsafe {
            from_glib(ffi::ges_project_add_asset(self.to_glib_none().0, asset.to_glib_none().0))
        }
    }

    fn add_encoding_profile<P: IsA<gst_pbutils::EncodingProfile>>(&self, profile: &P) -> bool {
        unsafe {
            from_glib(ffi::ges_project_add_encoding_profile(self.to_glib_none().0, profile.to_glib_none().0))
        }
    }

    fn create_asset<'a, P: Into<Option<&'a str>>>(&self, id: P, extractable_type: glib::types::Type) -> bool {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            from_glib(ffi::ges_project_create_asset(self.to_glib_none().0, id.0, extractable_type.to_glib()))
        }
    }

    fn create_asset_sync<'a, P: Into<Option<&'a str>>>(&self, id: P, extractable_type: glib::types::Type) -> Result<Option<Asset>, Error> {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_project_create_asset_sync(self.to_glib_none().0, id.0, extractable_type.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_asset(&self, id: &str, extractable_type: glib::types::Type) -> Option<Asset> {
        unsafe {
            from_glib_full(ffi::ges_project_get_asset(self.to_glib_none().0, id.to_glib_none().0, extractable_type.to_glib()))
        }
    }

    fn get_loading_assets(&self) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_project_get_loading_assets(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::ges_project_get_uri(self.to_glib_none().0))
        }
    }

    fn list_assets(&self, filter: glib::types::Type) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_project_list_assets(self.to_glib_none().0, filter.to_glib()))
        }
    }

    fn list_encoding_profiles(&self) -> Vec<gst_pbutils::EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_project_list_encoding_profiles(self.to_glib_none().0))
        }
    }

    fn load(&self, timeline: &Timeline) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_project_load(self.to_glib_none().0, timeline.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_asset<P: IsA<Asset>>(&self, asset: &P) -> bool {
        unsafe {
            from_glib(ffi::ges_project_remove_asset(self.to_glib_none().0, asset.to_glib_none().0))
        }
    }

    fn save<'a, P: IsA<Asset> + 'a, Q: Into<Option<&'a P>>>(&self, timeline: &Timeline, uri: &str, formatter_asset: Q, overwrite: bool) -> Result<(), Error> {
        let formatter_asset = formatter_asset.into();
        let formatter_asset = formatter_asset.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_project_save(self.to_glib_none().0, timeline.to_glib_none().0, uri.to_glib_none().0, formatter_asset.0, overwrite.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_asset_added<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Asset) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "asset-added",
                transmute(asset_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_asset_loading<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Asset) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "asset-loading",
                transmute(asset_loading_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_asset_removed<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Asset) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "asset-removed",
                transmute(asset_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_error_loading_asset<F: Fn(&Self, &Error, &str, glib::types::Type) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Error, &str, glib::types::Type) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "error-loading-asset",
                transmute(error_loading_asset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_loaded<F: Fn(&Self, &Timeline) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Timeline) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "loaded",
                transmute(loaded_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_missing_uri<F: Fn(&Self, &Error, &Asset) -> Option<String> + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Error, &Asset) -> Option<String> + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "missing-uri",
                transmute(missing_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn asset_added_trampoline<P>(this: *mut ffi::GESProject, asset: *mut ffi::GESAsset, f: glib_ffi::gpointer)
where P: IsA<Project> {
    let f: &&(Fn(&P, &Asset) + 'static) = transmute(f);
    f(&Project::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(asset))
}

unsafe extern "C" fn asset_loading_trampoline<P>(this: *mut ffi::GESProject, asset: *mut ffi::GESAsset, f: glib_ffi::gpointer)
where P: IsA<Project> {
    let f: &&(Fn(&P, &Asset) + 'static) = transmute(f);
    f(&Project::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(asset))
}

unsafe extern "C" fn asset_removed_trampoline<P>(this: *mut ffi::GESProject, asset: *mut ffi::GESAsset, f: glib_ffi::gpointer)
where P: IsA<Project> {
    let f: &&(Fn(&P, &Asset) + 'static) = transmute(f);
    f(&Project::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(asset))
}

unsafe extern "C" fn error_loading_asset_trampoline<P>(this: *mut ffi::GESProject, error: *mut glib_ffi::GError, id: *mut libc::c_char, extractable_type: glib_ffi::GType, f: glib_ffi::gpointer)
where P: IsA<Project> {
    let f: &&(Fn(&P, &Error, &str, glib::types::Type) + 'static) = transmute(f);
    f(&Project::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(error), &String::from_glib_none(id), from_glib(extractable_type))
}

unsafe extern "C" fn loaded_trampoline<P>(this: *mut ffi::GESProject, timeline: *mut ffi::GESTimeline, f: glib_ffi::gpointer)
where P: IsA<Project> {
    let f: &&(Fn(&P, &Timeline) + 'static) = transmute(f);
    f(&Project::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(timeline))
}

unsafe extern "C" fn missing_uri_trampoline<P>(this: *mut ffi::GESProject, error: *mut glib_ffi::GError, wrong_asset: *mut ffi::GESAsset, f: glib_ffi::gpointer) -> *mut libc::c_char
where P: IsA<Project> {
    let f: &&(Fn(&P, &Error, &Asset) -> Option<String> + 'static) = transmute(f);
    f(&Project::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(error), &from_glib_borrow(wrong_asset)).to_glib_full()
}
