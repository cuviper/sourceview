// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Completion;
use CompletionActivation;
use CompletionProposal;
use CompletionProvider;
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
use gtk;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CompletionContext(Object<ffi::GtkSourceCompletionContext, ffi::GtkSourceCompletionContextClass>);

    match fn {
        get_type => || ffi::gtk_source_completion_context_get_type(),
    }
}

pub trait CompletionContextExt {
    fn add_proposals<P: IsA<CompletionProvider>>(&self, provider: &P, proposals: &[CompletionProposal], finished: bool);

    fn get_activation(&self) -> CompletionActivation;

    fn get_iter(&self) -> Option<gtk::TextIter>;

    fn set_property_activation(&self, activation: CompletionActivation);

    fn get_property_completion(&self) -> Option<Completion>;

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancelled(&self);

    fn connect_property_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionContext> + IsA<glib::object::Object> + glib::object::ObjectExt> CompletionContextExt for O {
    fn add_proposals<P: IsA<CompletionProvider>>(&self, provider: &P, proposals: &[CompletionProposal], finished: bool) {
        unsafe {
            ffi::gtk_source_completion_context_add_proposals(self.to_glib_none().0, provider.to_glib_none().0, proposals.to_glib_none().0, finished.to_glib());
        }
    }

    fn get_activation(&self) -> CompletionActivation {
        unsafe {
            from_glib(ffi::gtk_source_completion_context_get_activation(self.to_glib_none().0))
        }
    }

    fn get_iter(&self) -> Option<gtk::TextIter> {
        unsafe {
            let mut iter = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_completion_context_get_iter(self.to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn set_property_activation(&self, activation: CompletionActivation) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "activation".to_glib_none().0, Value::from(&activation).to_glib_none().0);
        }
    }

    fn get_property_completion(&self) -> Option<Completion> {
        unsafe {
            let mut value = Value::from_type(<Completion as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "completion".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_cancelled(&self) {
        let _ = self.emit("cancelled", &[]).unwrap();
    }

    fn connect_property_activation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::activation",
                transmute(notify_activation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::completion",
                transmute(notify_completion_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::GtkSourceCompletionContext, f: glib_ffi::gpointer)
where P: IsA<CompletionContext> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_activation_trampoline<P>(this: *mut ffi::GtkSourceCompletionContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionContext> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_completion_trampoline<P>(this: *mut ffi::GtkSourceCompletionContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CompletionContext> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CompletionContext::from_glib_borrow(this).downcast_unchecked())
}
