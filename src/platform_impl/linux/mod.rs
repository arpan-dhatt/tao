// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

#![cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]

mod clipboard;
mod event_loop;
mod global_shortcut;
mod keyboard;
mod keycode;
mod menu;
mod monitor;
mod system_tray;
mod window;

pub use self::{
  clipboard::Clipboard,
  global_shortcut::{GlobalShortcut, ShortcutManager},
  keycode::{keycode_from_scancode, keycode_to_scancode},
  menu::{Menu, MenuItemAttributes},
  system_tray::{SystemTray, SystemTrayBuilder, TrayMenu},
};
pub use event_loop::{EventLoop, EventLoopProxy, EventLoopWindowTarget};
pub use monitor::{MonitorHandle, VideoMode};
pub use window::{hit_test, PlatformIcon, Window, WindowId};

use crate::keyboard::Key;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyEventExtra {
  pub text_with_all_modifiers: Option<&'static str>,
  pub key_without_modifiers: Key<'static>,
}

#[derive(Clone)]
pub struct PlatformSpecificWindowBuilderAttributes {
  pub skip_taskbar: bool,
}

impl Default for PlatformSpecificWindowBuilderAttributes {
  fn default() -> Self {
    Self {
      skip_taskbar: false,
    }
  }
}

unsafe impl Send for PlatformSpecificWindowBuilderAttributes {}
unsafe impl Sync for PlatformSpecificWindowBuilderAttributes {}

#[derive(Debug, Clone)]
pub struct OsError;

impl std::fmt::Display for OsError {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    Ok(())
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(usize);

impl DeviceId {
  pub unsafe fn dummy() -> Self {
    Self(0)
  }
}
