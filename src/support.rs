#![allow(dead_code)]
#![allow(unused_variables)]

use std::num::NonZeroU32;

use fltk::prelude::WindowExt;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use glutin::config::{Api, ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder};
use glutin::display::{Display, DisplayApiPreference};
use glutin::surface::{SurfaceAttributes, SurfaceAttributesBuilder, WindowSurface};

use crate::RawWindowHandleExt;

/// Create template to find OpenGL config.
pub fn config_template(api: Api,raw_window_handle: RawWindowHandle) -> ConfigTemplate {
    ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        // .with_transparency(true)
        // .with_multisampling(8)
        .compatible_with_native_window(raw_window_handle)
        .with_surface_type(ConfigSurfaceTypes::WINDOW)
        .build()
}

/// Create surface attributes for window surface.
pub fn surface_attributes<W: WindowExt + RawWindowHandleExt>(
    window: &W,
) -> SurfaceAttributes<WindowSurface> {
    let raw_window_handle = window.raw_window_handle();
    SurfaceAttributesBuilder::<WindowSurface>::new()
        .with_srgb(Some(true))
        .build(
            raw_window_handle,
            NonZeroU32::new(window.width() as _).unwrap(),
            NonZeroU32::new(window.height() as _).unwrap(),
        )
}

/// Create the display.
pub fn create_display(
    raw_display: RawDisplayHandle,
    raw_window_handle: RawWindowHandle,
) -> Display {
    #[cfg(egl_backend)]
    let preference = DisplayApiPreference::Egl;

    #[cfg(glx_backend)]
    let preference = DisplayApiPreference::Glx(Box::new(register_xlib_error_hook));

    #[cfg(cgl_backend)]
    let preference = DisplayApiPreference::Cgl;

    #[cfg(wgl_backend)]
    let preference = DisplayApiPreference::Wgl(Some(raw_window_handle));

    #[cfg(all(egl_backend, wgl_backend))]
    let preference = DisplayApiPreference::WglThenEgl(Some(raw_window_handle));

    #[cfg(all(egl_backend, glx_backend))]
    let preference = DisplayApiPreference::GlxThenEgl(Box::new(register_xlib_error_hook));

    // Create connection to underlying OpenGL client Api.
    unsafe { Display::from_raw(raw_display, preference).unwrap() }
}

pub type XlibErrorHook =
    Box<dyn Fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> bool + Send + Sync>;

pub fn register_xlib_error_hook(_hook: XlibErrorHook) {}