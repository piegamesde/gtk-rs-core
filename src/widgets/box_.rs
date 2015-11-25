// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

use glib::object::{Downcast, Upcast};
use super::widget::Widget;
use {
    Orientation,
    PackType,
};

glib_wrapper! {
    pub struct Box(Object<ffi::GtkBox>): Widget, ::Container, ::Orientable, ::Buildable;

    match fn {
        get_type => || ffi::gtk_box_get_type(),
    }
}

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Box {
        unsafe {
            Widget::from_glib_none(ffi::gtk_box_new(orientation, spacing)).downcast_unchecked()
        }
    }
}

pub trait BoxExt {
    fn pack_start<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32);
    fn pack_end<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32);
    fn get_homogeneous(&self) -> bool;
    fn set_homogeneouse(&self, homogeneous: bool);
    fn get_spacing(&self) -> i32;
    fn set_spacing(&self, spacing: i32);
    fn reorder_child<T: Upcast<Widget>>(&self, child: &T, position: i32);
    fn query_child_packing<T: Upcast<Widget>>(&self, child: &T) -> (bool, bool, u32, PackType);
    fn set_child_packing<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool,
                                            padding: u32, pack_type: PackType);
}

impl<O: Upcast<Box>> BoxExt for O {
    fn pack_start<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_start(
                self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(),
                fill.to_glib(), padding);
        }
    }

    fn pack_end<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_end(
                self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(),
                fill.to_glib(), padding);
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_box_get_homogeneous(self.to_glib_none().0)) }
    }

    fn set_homogeneouse(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(self.to_glib_none().0)
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    fn reorder_child<T: Upcast<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_box_reorder_child(
                self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    fn query_child_packing<T: Upcast<Widget>>(&self, child: &T) -> (bool, bool, u32, PackType) {
        let mut c_expand = 0;
        let mut c_padding = 0;
        let mut c_fill = 0;
        let mut pack_type = PackType::Start;
        unsafe {
            ffi::gtk_box_query_child_packing(
                self.to_glib_none().0, child.to_glib_none().0, &mut c_expand,
                &mut c_fill, &mut c_padding, &mut pack_type);
        }
        (from_glib(c_expand), from_glib(c_fill), c_padding, pack_type)
    }

    fn set_child_packing<T: Upcast<Widget>>(&self, child: &T, expand: bool, fill: bool,
                                            padding: u32, pack_type: PackType) {
        unsafe {
            ffi::gtk_box_set_child_packing(
                self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(),
                fill.to_glib(), padding, pack_type);
        }
    }
}
