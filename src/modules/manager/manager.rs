//! This module provides an interface for working with the screen. With that I mean that you can get or wirte to the handle of the current screen. stdout.
//! Because crossterm can work with alternate screen, we need a place that holds the handle to the current screen so we can write to that screen.

use super::*;

use std::any::Any;
use std::fmt::Display;
use std::io::{self, Write};

#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

/// Struct that stores an specific platform implementation for screen related actions.
pub struct ScreenManager {
    screen_manager: Box<IScreenManager>,
}

impl ScreenManager {
    /// Create new screen manager instance whereon screen related actions can be performed.
    pub fn new() -> ScreenManager {
        #[cfg(target_os = "windows")]
        let screen_manager = functions::get_module::<Box<IScreenManager>>(
            Box::from(WinApiScreenManager::new()),
            Box::from(AnsiScreenManager::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let screen_manager = Box::from(AnsiScreenManager::new()) as Box<IScreenManager>;

        ScreenManager {
            screen_manager: screen_manager,
        }
    }

    /// Set whether screen is raw screen.
    pub fn set_is_raw_screen(&mut self, value: bool) {
        self.screen_manager.set_is_raw_screen(value);
    }

    /// Set whether the current screen is alternate screen.
    pub fn set_is_alternate_screen(&mut self, value: bool) {
        self.screen_manager.set_is_alternate_screen(value);
    }

    /// Check if the current screen is in rawscreen modes
    pub fn is_raw_screen(&self) -> bool {
        self.screen_manager.is_raw_screen()
    }

    /// Check if the current screen is in alternate modes.
    pub fn is_alternate_screen(&self) -> bool {
        self.screen_manager.is_alternate_screen()
    }

    /// Write String to the current screen.
    pub fn write_string(&self, string: String) -> io::Result<usize> {
        self.screen_manager.write_str(string.as_str())
    }

    /// Flush the current screen.
    pub fn flush(&self) -> io::Result<()>
    {
        self.screen_manager.flush()
    }

    /// Write &str to the current screen.
    pub fn write_str(&self, string: &str) -> io::Result<usize> {
        self.screen_manager.write_str(string)
    }

    /// Can be used to get an specific implementation used for the current platform.
    pub fn as_any(&self) -> &Any {
        self.screen_manager.as_any()
    }

    /// Can be used to get an specific implementation used for the current platform.
    pub fn as_any_mut(&mut self) -> &mut Any {
        self.screen_manager.as_any_mut()
    }
}