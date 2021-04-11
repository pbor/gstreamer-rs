// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PluginFeature;
use crate::Rank;

use glib::object::{Cast, IsA};
use glib::translate::{from_glib, FromGlibPtrFull, ToGlib, ToGlibPtr};

pub trait PluginFeatureExtManual: Sized + 'static {
    fn rank(&self) -> Rank;
    fn set_rank(&self, rank: Rank);
    fn load(&self) -> Result<Self, glib::BoolError>;
}

impl<O: IsA<PluginFeature>> PluginFeatureExtManual for O {
    fn rank(&self) -> Rank {
        unsafe {
            let rank = ffi::gst_plugin_feature_get_rank(self.as_ref().to_glib_none().0);
            from_glib(rank as i32)
        }
    }

    fn set_rank(&self, rank: Rank) {
        unsafe {
            ffi::gst_plugin_feature_set_rank(self.as_ref().to_glib_none().0, rank.to_glib() as u32);
        }
    }

    fn load(&self) -> Result<Self, glib::BoolError> {
        unsafe {
            let loaded = Option::<PluginFeature>::from_glib_full(ffi::gst_plugin_feature_load(
                self.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to load plugin feature"))?;
            Ok(loaded.unsafe_cast())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_load() {
        crate::init().unwrap();

        let factory = crate::ElementFactory::find("identity").unwrap();
        let loaded = factory.load().unwrap();
        assert_eq!(factory.type_(), loaded.type_());
        let _element = loaded.create(None).unwrap();
    }
}
