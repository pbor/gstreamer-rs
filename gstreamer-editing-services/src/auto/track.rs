// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Timeline;
use TrackElement;
use TrackType;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Track(Object<ffi::GESTrack, ffi::GESTrackClass>): [
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::ges_track_get_type(),
    }
}

impl Track {
    pub fn new(type_: TrackType, caps: &gst::Caps) -> Track {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_track_new(type_.to_glib(), caps.to_glib_full()))
        }
    }
}

pub trait GESTrackExt {
    fn add_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError>;

    fn commit(&self) -> bool;

    fn get_caps(&self) -> Option<gst::Caps>;

    fn get_elements(&self) -> Vec<TrackElement>;

    fn get_mixing(&self) -> bool;

    fn get_timeline(&self) -> Option<Timeline>;

    fn remove_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError>;

    //fn set_create_element_for_gap_func(&self, func: /*Unknown conversion*//*Unimplemented*/CreateElementForGapFunc);

    fn set_mixing(&self, mixing: bool);

    fn set_restriction_caps(&self, caps: &gst::Caps);

    fn set_timeline(&self, timeline: &Timeline);

    fn update_restriction_caps(&self, caps: &gst::Caps);

    fn get_property_duration(&self) -> u64;

    fn get_property_restriction_caps(&self) -> Option<gst::Caps>;

    fn get_property_track_type(&self) -> TrackType;

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Track> + IsA<glib::object::Object>> GESTrackExt for O {
    fn add_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::ges_track_add_element(self.to_glib_none().0, object.to_glib_none().0), "Failed to add element")
        }
    }

    fn commit(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_commit(self.to_glib_none().0))
        }
    }

    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_none(ffi::ges_track_get_caps(self.to_glib_none().0))
        }
    }

    fn get_elements(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_track_get_elements(self.to_glib_none().0))
        }
    }

    fn get_mixing(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_get_mixing(self.to_glib_none().0))
        }
    }

    fn get_timeline(&self) -> Option<Timeline> {
        unsafe {
            from_glib_none(ffi::ges_track_get_timeline(self.to_glib_none().0))
        }
    }

    fn remove_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::ges_track_remove_element(self.to_glib_none().0, object.to_glib_none().0), "Failed to remove element")
        }
    }

    //fn set_create_element_for_gap_func(&self, func: /*Unknown conversion*//*Unimplemented*/CreateElementForGapFunc) {
    //    unsafe { TODO: call ffi::ges_track_set_create_element_for_gap_func() }
    //}

    fn set_mixing(&self, mixing: bool) {
        unsafe {
            ffi::ges_track_set_mixing(self.to_glib_none().0, mixing.to_glib());
        }
    }

    fn set_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::ges_track_set_restriction_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    fn set_timeline(&self, timeline: &Timeline) {
        unsafe {
            ffi::ges_track_set_timeline(self.to_glib_none().0, timeline.to_glib_none().0);
        }
    }

    fn update_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::ges_track_update_restriction_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    fn get_property_duration(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "duration".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_restriction_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let mut value = Value::from_type(<gst::Caps as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "restriction-caps".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_track_type(&self) -> TrackType {
        unsafe {
            let mut value = Value::from_type(<TrackType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "track-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "commited",
                transmute(commited_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TrackElement) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "track-element-added",
                transmute(track_element_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TrackElement) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "track-element-removed",
                transmute(track_element_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::duration",
                transmute(notify_duration_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mixing",
                transmute(notify_mixing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::restriction-caps",
                transmute(notify_restriction_caps_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn commited_trampoline<P>(this: *mut ffi::GESTrack, f: glib_ffi::gpointer)
where P: IsA<Track> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Track::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn track_element_added_trampoline<P>(this: *mut ffi::GESTrack, effect: *mut ffi::GESTrackElement, f: glib_ffi::gpointer)
where P: IsA<Track> {
    let f: &&(Fn(&P, &TrackElement) + 'static) = transmute(f);
    f(&Track::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(effect))
}

unsafe extern "C" fn track_element_removed_trampoline<P>(this: *mut ffi::GESTrack, effect: *mut ffi::GESTrackElement, f: glib_ffi::gpointer)
where P: IsA<Track> {
    let f: &&(Fn(&P, &TrackElement) + 'static) = transmute(f);
    f(&Track::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(effect))
}

unsafe extern "C" fn notify_duration_trampoline<P>(this: *mut ffi::GESTrack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Track> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Track::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mixing_trampoline<P>(this: *mut ffi::GESTrack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Track> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Track::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_restriction_caps_trampoline<P>(this: *mut ffi::GESTrack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Track> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Track::from_glib_borrow(this).downcast_unchecked())
}
