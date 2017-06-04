// This file was generated by gir (fdeaa47) from gir-files (2e2a9ca)
// DO NOT EDIT

use CompletionContext;
use CompletionInfo;
use CompletionProvider;
use Error;
use View;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Completion(Object<ffi::GtkSourceCompletion>);

    match fn {
        get_type => || ffi::gtk_source_completion_get_type(),
    }
}

pub trait CompletionExt {
    fn add_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), Error>;

    fn block_interactive(&self);

    fn get_info_window(&self) -> Option<CompletionInfo>;

    fn get_providers(&self) -> Vec<CompletionProvider>;

    fn get_view(&self) -> Option<View>;

    fn hide(&self);

    fn move_window(&self, iter: &mut gtk::TextIter);

    fn remove_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), Error>;

    fn show(&self, providers: &[CompletionProvider], context: &CompletionContext) -> bool;

    fn unblock_interactive(&self);

    fn get_property_accelerators(&self) -> u32;

    fn set_property_accelerators(&self, accelerators: u32);

    fn get_property_auto_complete_delay(&self) -> u32;

    fn set_property_auto_complete_delay(&self, auto_complete_delay: u32);

    fn get_property_proposal_page_size(&self) -> u32;

    fn set_property_proposal_page_size(&self, proposal_page_size: u32);

    fn get_property_provider_page_size(&self) -> u32;

    fn set_property_provider_page_size(&self, provider_page_size: u32);

    fn get_property_remember_info_visibility(&self) -> bool;

    fn set_property_remember_info_visibility(&self, remember_info_visibility: bool);

    fn get_property_select_on_show(&self) -> bool;

    fn set_property_select_on_show(&self, select_on_show: bool);

    fn get_property_show_headers(&self) -> bool;

    fn set_property_show_headers(&self, show_headers: bool);

    fn get_property_show_icons(&self) -> bool;

    fn set_property_show_icons(&self, show_icons: bool);

    fn connect_activate_proposal<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_move_cursor<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(&self, f: F) -> u64;

    fn connect_move_page<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(&self, f: F) -> u64;

    fn connect_populate_context<F: Fn(&Self, &CompletionContext) + 'static>(&self, f: F) -> u64;

    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Completion> + IsA<glib::object::Object>> CompletionExt for O {
    fn add_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_completion_add_provider(self.to_glib_none().0, provider.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn block_interactive(&self) {
        unsafe {
            ffi::gtk_source_completion_block_interactive(self.to_glib_none().0);
        }
    }

    fn get_info_window(&self) -> Option<CompletionInfo> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_get_info_window(self.to_glib_none().0))
        }
    }

    fn get_providers(&self) -> Vec<CompletionProvider> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_completion_get_providers(self.to_glib_none().0))
        }
    }

    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_get_view(self.to_glib_none().0))
        }
    }

    fn hide(&self) {
        unsafe {
            ffi::gtk_source_completion_hide(self.to_glib_none().0);
        }
    }

    fn move_window(&self, iter: &mut gtk::TextIter) {
        unsafe {
            ffi::gtk_source_completion_move_window(self.to_glib_none().0, iter.to_glib_none_mut().0);
        }
    }

    fn remove_provider<P: IsA<CompletionProvider>>(&self, provider: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_source_completion_remove_provider(self.to_glib_none().0, provider.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn show(&self, providers: &[CompletionProvider], context: &CompletionContext) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_show(self.to_glib_none().0, providers.to_glib_none().0, context.to_glib_none().0))
        }
    }

    fn unblock_interactive(&self) {
        unsafe {
            ffi::gtk_source_completion_unblock_interactive(self.to_glib_none().0);
        }
    }

    fn get_property_accelerators(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accelerators".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_accelerators(&self, accelerators: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accelerators".to_glib_none().0, Value::from(&accelerators).to_glib_none().0);
        }
    }

    fn get_property_auto_complete_delay(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "auto-complete-delay".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_auto_complete_delay(&self, auto_complete_delay: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "auto-complete-delay".to_glib_none().0, Value::from(&auto_complete_delay).to_glib_none().0);
        }
    }

    fn get_property_proposal_page_size(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "proposal-page-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_proposal_page_size(&self, proposal_page_size: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "proposal-page-size".to_glib_none().0, Value::from(&proposal_page_size).to_glib_none().0);
        }
    }

    fn get_property_provider_page_size(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "provider-page-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_provider_page_size(&self, provider_page_size: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "provider-page-size".to_glib_none().0, Value::from(&provider_page_size).to_glib_none().0);
        }
    }

    fn get_property_remember_info_visibility(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "remember-info-visibility".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_remember_info_visibility(&self, remember_info_visibility: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "remember-info-visibility".to_glib_none().0, Value::from(&remember_info_visibility).to_glib_none().0);
        }
    }

    fn get_property_select_on_show(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "select-on-show".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_select_on_show(&self, select_on_show: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "select-on-show".to_glib_none().0, Value::from(&select_on_show).to_glib_none().0);
        }
    }

    fn get_property_show_headers(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-headers".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_headers(&self, show_headers: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-headers".to_glib_none().0, Value::from(&show_headers).to_glib_none().0);
        }
    }

    fn get_property_show_icons(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-icons".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_icons(&self, show_icons: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-icons".to_glib_none().0, Value::from(&show_icons).to_glib_none().0);
        }
    }

    fn connect_activate_proposal<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-proposal",
                transmute(activate_proposal_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "hide",
                transmute(hide_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_cursor<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, gtk::ScrollStep, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_page<F: Fn(&Self, gtk::ScrollStep, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, gtk::ScrollStep, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-page",
                transmute(move_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_populate_context<F: Fn(&Self, &CompletionContext) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &CompletionContext) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "populate-context",
                transmute(populate_context_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show",
                transmute(show_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_proposal_trampoline<P>(this: *mut ffi::GtkSourceCompletion, f: glib_ffi::gpointer)
where P: IsA<Completion> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Completion::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn hide_trampoline<P>(this: *mut ffi::GtkSourceCompletion, f: glib_ffi::gpointer)
where P: IsA<Completion> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Completion::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn move_cursor_trampoline<P>(this: *mut ffi::GtkSourceCompletion, step: gtk_ffi::GtkScrollStep, num: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Completion> {
    callback_guard!();
    let f: &Box_<Fn(&P, gtk::ScrollStep, i32) + 'static> = transmute(f);
    f(&Completion::from_glib_none(this).downcast_unchecked(), from_glib(step), num)
}

unsafe extern "C" fn move_page_trampoline<P>(this: *mut ffi::GtkSourceCompletion, step: gtk_ffi::GtkScrollStep, num: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Completion> {
    callback_guard!();
    let f: &Box_<Fn(&P, gtk::ScrollStep, i32) + 'static> = transmute(f);
    f(&Completion::from_glib_none(this).downcast_unchecked(), from_glib(step), num)
}

unsafe extern "C" fn populate_context_trampoline<P>(this: *mut ffi::GtkSourceCompletion, context: *mut ffi::GtkSourceCompletionContext, f: glib_ffi::gpointer)
where P: IsA<Completion> {
    callback_guard!();
    let f: &Box_<Fn(&P, &CompletionContext) + 'static> = transmute(f);
    f(&Completion::from_glib_none(this).downcast_unchecked(), &from_glib_none(context))
}

unsafe extern "C" fn show_trampoline<P>(this: *mut ffi::GtkSourceCompletion, f: glib_ffi::gpointer)
where P: IsA<Completion> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Completion::from_glib_none(this).downcast_unchecked())
}