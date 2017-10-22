// This file was generated by gir (12a28ac) from gir-files (469db10)
// DO NOT EDIT

use Style;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct StyleScheme(Object<ffi::GtkSourceStyleScheme>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_get_type(),
    }
}

pub trait StyleSchemeExt {
    fn get_authors(&self) -> Vec<String>;

    fn get_description(&self) -> Option<String>;

    fn get_filename(&self) -> Option<String>;

    fn get_id(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_style(&self, style_id: &str) -> Option<Style>;

    fn get_property_description(&self) -> Option<String>;

    fn get_property_filename(&self) -> Option<String>;

    fn get_property_id(&self) -> Option<String>;

    fn get_property_name(&self) -> Option<String>;

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleScheme> + IsA<glib::object::Object>> StyleSchemeExt for O {
    fn get_authors(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_get_authors(self.to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_description(self.to_glib_none().0))
        }
    }

    fn get_filename(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_filename(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_id(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_name(self.to_glib_none().0))
        }
    }

    fn get_style(&self, style_id: &str) -> Option<Style> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_get_style(self.to_glib_none().0, style_id.to_glib_none().0))
        }
    }

    fn get_property_description(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "description".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_filename(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "filename".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::description",
                transmute(notify_description_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::filename",
                transmute(notify_filename_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::id",
                transmute(notify_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_description_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_filename_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_id_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GtkSourceStyleScheme, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleScheme> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleScheme::from_glib_borrow(this).downcast_unchecked())
}
