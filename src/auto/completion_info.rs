// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CompletionInfo(Object<ffi::GtkSourceCompletionInfo, ffi::GtkSourceCompletionInfoClass>): [
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::gtk_source_completion_info_get_type(),
    }
}

impl CompletionInfo {
    pub fn new() -> CompletionInfo {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_info_new())
        }
    }
}

impl Default for CompletionInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CompletionInfoExt {
    #[cfg_attr(feature = "v3_8", deprecated)]
    fn get_widget(&self) -> Option<gtk::Widget>;

    #[cfg_attr(feature = "v3_8", deprecated)]
    fn set_widget<'a, P: IsA<gtk::Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_before_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn emit_before_show(&self);
}

impl<O: IsA<CompletionInfo> + IsA<glib::object::Object> + glib::object::ObjectExt> CompletionInfoExt for O {
    fn get_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_info_get_widget(self.to_glib_none().0))
        }
    }

    fn set_widget<'a, P: IsA<gtk::Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_source_completion_info_set_widget(self.to_glib_none().0, widget.0);
        }
    }

    fn connect_before_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "before-show",
                transmute(before_show_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_before_show(&self) {
        let _ = self.emit("before-show", &[]).unwrap();
    }
}

unsafe extern "C" fn before_show_trampoline<P>(this: *mut ffi::GtkSourceCompletionInfo, f: glib_ffi::gpointer)
where P: IsA<CompletionInfo> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionInfo::from_glib_borrow(this).downcast_unchecked())
}
