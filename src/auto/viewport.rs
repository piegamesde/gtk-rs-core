// This file was generated by gir (3294959) from gir-files (0bcaef9)
// DO NOT EDIT

use Adjustment;
use Bin;
use Container;
use Scrollable;
use ShadowType;
use Widget;
use ffi;
use gdk;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Viewport(Object<ffi::GtkViewport>): Bin, Container, Widget, Scrollable;

    match fn {
        get_type => || ffi::gtk_viewport_get_type(),
    }
}

impl Viewport {
    pub fn new<'a, 'b, P: Into<Option<&'a Adjustment>>, Q: Into<Option<&'b Adjustment>>>(hadjustment: P, vadjustment: Q) -> Viewport {
        assert_initialized_main_thread!();
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_viewport_new(hadjustment.0, vadjustment.0)).downcast_unchecked()
        }
    }
}

pub trait ViewportExt {
    fn get_bin_window(&self) -> Option<gdk::Window>;

    fn get_shadow_type(&self) -> ShadowType;

    fn get_view_window(&self) -> Option<gdk::Window>;

    fn set_shadow_type(&self, type_: ShadowType);
}

impl<O: IsA<Viewport>> ViewportExt for O {
    fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_bin_window(self.to_glib_none().0))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_viewport_get_shadow_type(self.to_glib_none().0))
        }
    }

    fn get_view_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_view_window(self.to_glib_none().0))
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_viewport_set_shadow_type(self.to_glib_none().0, type_.to_glib());
        }
    }
}
