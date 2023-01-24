#![warn(missing_debug_implementations)]
#![allow(clippy::single_match)]
// Hides console window on windows
#![windows_subsystem = "windows"]

pub(crate) mod config;
pub(crate) mod dialogs;
pub(crate) mod globals;
pub(crate) mod utils;

/// Widgets
mod app;
mod appmenu;
mod appwindow;
mod canvas;
mod canvasmenu;
mod canvaswrapper;
mod colorpicker;
mod iconpicker;
mod mainheader;
mod overlays;
pub(crate) mod penssidebar;
mod settingspanel;
pub(crate) mod strokewidthpicker;
mod unitentry;
mod workspacebrowser;

// Re-exports
pub(crate) use app::RnoteApp;
pub(crate) use appmenu::AppMenu;
pub(crate) use appwindow::RnoteAppWindow;
pub(crate) use canvas::RnoteCanvas;
pub(crate) use canvasmenu::CanvasMenu;
pub(crate) use canvaswrapper::RnoteCanvasWrapper;
pub(crate) use colorpicker::ColorPicker;
pub(crate) use iconpicker::IconPicker;
pub(crate) use mainheader::MainHeader;
pub(crate) use overlays::RnoteOverlays;
pub(crate) use penssidebar::PensSideBar;
pub(crate) use settingspanel::SettingsPanel;
pub(crate) use strokewidthpicker::StrokeWidthPicker;
pub(crate) use unitentry::UnitEntry;
pub(crate) use workspacebrowser::WorkspaceBrowser;

extern crate nalgebra as na;
extern crate parry2d_f64 as p2d;

use gtk4::prelude::*;

fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    setup_windows_env();
    #[cfg(target_os = "macos")]
    if let Err(e) = setup_macos_env() {
        eprintln!("failed to setup env for macos, Err: {e:?}");
    }

    let app = RnoteApp::new();
    app.run();

    Ok(())
}

/// we need to set some env vars on windows
#[cfg(target_os = "windows")]
fn setup_windows_env() {
    std::env::set_var(
        "GSETTINGS_SCHEMA_DIR",
        config::DATADIR.to_string() + "/glib-2.0/schemas",
    );
    std::env::set_var(
        "GDK_PIXBUF_MODULEDIR",
        config::LIBDIR.to_string() + "/gdk-pixbuf-2.0/2.10.0/loaders",
    );
}

/// we need to set some env vars for macos app bundles
#[cfg(target_os = "macos")]
fn setup_macos_env() -> anyhow::Result<()> {
    use std::ffi::OsStr;
    use std::path::{Component, PathBuf};

    let current_dir = std::env::current_dir()?.canonicalize()?;
    if current_dir
        .components()
        .zip(current_dir.components().skip(1))
        .any(|(a, b)| {
            if let (Component::Normal(a), Component::Normal(b)) = (a, b) {
                a == OsStr::new("Contents") && b == OsStr::new("MacOS")
            } else {
                false
            }
        })
    {
        std::env::set_var("XDG_DATA_DIRS", &current_dir.join("/..Resources/share"));
        std::env::set_var(
            "GDK_PIXBUF_MODULE_FILE",
            current_dir.join(PathBuf::from(
                "/../Resources/lib/gdk-pixbuf-2.0/2.10.0/loaders/loaders.cache",
            )),
        );
    }
    Ok(())
}
