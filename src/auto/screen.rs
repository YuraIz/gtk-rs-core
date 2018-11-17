// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Display;
use Rectangle;
use Visual;
use Window;
use cairo;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Screen(Object<ffi::GdkScreen>);

    match fn {
        get_type => || ffi::gdk_screen_get_type(),
    }
}

impl Screen {
    pub fn get_default() -> Option<Screen> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_screen_get_default())
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn height() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_height()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn height_mm() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_height_mm()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn width() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_width()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn width_mm() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_screen_width_mm()
        }
    }
}

pub trait ScreenExt {
    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_active_window(&self) -> Option<Window>;

    fn get_display(&self) -> Display;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_height(&self) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_height_mm(&self) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_at_point(&self, x: i32, y: i32) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_at_window(&self, window: &Window) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_geometry(&self, monitor_num: i32) -> Rectangle;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_height_mm(&self, monitor_num: i32) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<String>;

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_width_mm(&self, monitor_num: i32) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_monitor_workarea(&self, monitor_num: i32) -> Rectangle;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_n_monitors(&self) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_number(&self) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_primary_monitor(&self) -> i32;

    fn get_resolution(&self) -> f64;

    fn get_rgba_visual(&self) -> Option<Visual>;

    fn get_root_window(&self) -> Option<Window>;

    fn get_setting(&self, name: &str, value: &mut glib::Value) -> bool;

    fn get_system_visual(&self) -> Option<Visual>;

    fn get_toplevel_windows(&self) -> Vec<Window>;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_width(&self) -> i32;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn get_width_mm(&self) -> i32;

    fn get_window_stack(&self) -> Vec<Window>;

    fn is_composited(&self) -> bool;

    fn list_visuals(&self) -> Vec<Visual>;

    #[cfg_attr(feature = "v3_22", deprecated)]
    fn make_display_name(&self) -> String;

    fn set_font_options<'a, P: Into<Option<&'a cairo::FontOptions>>>(&self, options: P);

    fn set_resolution(&self, dpi: f64);

    //fn get_property_font_options(&self) -> /*Unimplemented*/Fundamental: Pointer;

    fn connect_composited_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_monitors_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_size_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Screen> + IsA<glib::object::Object>> ScreenExt for O {
    fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_full(ffi::gdk_screen_get_active_window(self.to_glib_none().0))
        }
    }

    fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_display(self.to_glib_none().0))
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_height(self.to_glib_none().0)
        }
    }

    fn get_height_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_height_mm(self.to_glib_none().0)
        }
    }

    fn get_monitor_at_point(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_at_point(self.to_glib_none().0, x, y)
        }
    }

    fn get_monitor_at_window(&self, window: &Window) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_at_window(self.to_glib_none().0, window.to_glib_none().0)
        }
    }

    fn get_monitor_geometry(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            ffi::gdk_screen_get_monitor_geometry(self.to_glib_none().0, monitor_num, dest.to_glib_none_mut().0);
            dest
        }
    }

    fn get_monitor_height_mm(&self, monitor_num: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_height_mm(self.to_glib_none().0, monitor_num)
        }
    }

    fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gdk_screen_get_monitor_plug_name(self.to_glib_none().0, monitor_num))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_scale_factor(self.to_glib_none().0, monitor_num)
        }
    }

    fn get_monitor_width_mm(&self, monitor_num: i32) -> i32 {
        unsafe {
            ffi::gdk_screen_get_monitor_width_mm(self.to_glib_none().0, monitor_num)
        }
    }

    fn get_monitor_workarea(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            ffi::gdk_screen_get_monitor_workarea(self.to_glib_none().0, monitor_num, dest.to_glib_none_mut().0);
            dest
        }
    }

    fn get_n_monitors(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_n_monitors(self.to_glib_none().0)
        }
    }

    fn get_number(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_number(self.to_glib_none().0)
        }
    }

    fn get_primary_monitor(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_primary_monitor(self.to_glib_none().0)
        }
    }

    fn get_resolution(&self) -> f64 {
        unsafe {
            ffi::gdk_screen_get_resolution(self.to_glib_none().0)
        }
    }

    fn get_rgba_visual(&self) -> Option<Visual> {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_rgba_visual(self.to_glib_none().0))
        }
    }

    fn get_root_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_root_window(self.to_glib_none().0))
        }
    }

    fn get_setting(&self, name: &str, value: &mut glib::Value) -> bool {
        unsafe {
            from_glib(ffi::gdk_screen_get_setting(self.to_glib_none().0, name.to_glib_none().0, value.to_glib_none_mut().0))
        }
    }

    fn get_system_visual(&self) -> Option<Visual> {
        unsafe {
            from_glib_none(ffi::gdk_screen_get_system_visual(self.to_glib_none().0))
        }
    }

    fn get_toplevel_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_screen_get_toplevel_windows(self.to_glib_none().0))
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_width(self.to_glib_none().0)
        }
    }

    fn get_width_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_screen_get_width_mm(self.to_glib_none().0)
        }
    }

    fn get_window_stack(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gdk_screen_get_window_stack(self.to_glib_none().0))
        }
    }

    fn is_composited(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_screen_is_composited(self.to_glib_none().0))
        }
    }

    fn list_visuals(&self) -> Vec<Visual> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_screen_list_visuals(self.to_glib_none().0))
        }
    }

    fn make_display_name(&self) -> String {
        unsafe {
            from_glib_full(ffi::gdk_screen_make_display_name(self.to_glib_none().0))
        }
    }

    fn set_font_options<'a, P: Into<Option<&'a cairo::FontOptions>>>(&self, options: P) {
        let options = options.into();
        let options = options.to_glib_none();
        unsafe {
            ffi::gdk_screen_set_font_options(self.to_glib_none().0, options.0);
        }
    }

    fn set_resolution(&self, dpi: f64) {
        unsafe {
            ffi::gdk_screen_set_resolution(self.to_glib_none().0, dpi);
        }
    }

    //fn get_property_font_options(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "font-options".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    fn connect_composited_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "composited-changed",
                transmute(composited_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_monitors_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "monitors-changed",
                transmute(monitors_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_size_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "size-changed",
                transmute(size_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_font_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::font-options",
                transmute(notify_font_options_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::resolution",
                transmute(notify_resolution_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn composited_changed_trampoline<P>(this: *mut ffi::GdkScreen, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Screen::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn monitors_changed_trampoline<P>(this: *mut ffi::GdkScreen, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Screen::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn size_changed_trampoline<P>(this: *mut ffi::GdkScreen, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Screen::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_font_options_trampoline<P>(this: *mut ffi::GdkScreen, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Screen::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_resolution_trampoline<P>(this: *mut ffi::GdkScreen, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Screen> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Screen::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Screen")
    }
}
