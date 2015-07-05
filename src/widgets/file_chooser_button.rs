// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A button to launch a file selection dialog
//! # Available signals:
//! * `file-set`: Action

use ffi;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::to_gboolean;
use Dialog;
use cast::{GTK_FILE_CHOOSER_BUTTON};
use libc::c_int;

struct_Widget!(FileChooserButton);

impl FileChooserButton {
    pub fn new(title: &str, action: ::FileChooserAction) -> Option<FileChooserButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_file_chooser_button_new(title.to_glib_none().0,
                                             action)
        };
        check_pointer!(tmp_pointer, FileChooserButton)
    }

    pub fn new_with_dialog(dialog: &Dialog) -> Option<FileChooserButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_file_chooser_button_new_with_dialog(dialog.unwrap_pointer())
        };
        check_pointer!(tmp_pointer, FileChooserButton)
    }

    pub fn get_title(&self) -> Option<String> {
        let p = GTK_FILE_CHOOSER_BUTTON(self.pointer);

        unsafe {
            from_glib_none(ffi::gtk_file_chooser_button_get_title(p))
        }
    }

    pub fn set_title(&self, title: &str) {
        let p = GTK_FILE_CHOOSER_BUTTON(self.pointer);

        unsafe {
            ffi::gtk_file_chooser_button_set_title(p, title.to_glib_none().0);
        }
    }

    pub fn get_width_chars(&self) -> i32 {
        let p = GTK_FILE_CHOOSER_BUTTON(self.pointer);

        unsafe {
            ffi::gtk_file_chooser_button_get_width_chars(p) as i32
        }
    }

    pub fn set_width_chars(&self, n_chars: i32) {
        let p = GTK_FILE_CHOOSER_BUTTON(self.pointer);

        unsafe {
            ffi::gtk_file_chooser_button_set_width_chars(p, n_chars as c_int);
        }
    }

    pub fn get_focus_on_click(&self) -> bool {
        let p = GTK_FILE_CHOOSER_BUTTON(self.pointer);

        unsafe {
            ffi::gtk_file_chooser_button_get_focus_on_click(p) != 0
        }
    }

    pub fn set_focus_on_click(&self, focus_on_click: bool) {
        let p = GTK_FILE_CHOOSER_BUTTON(self.pointer);

        unsafe {
            ffi::gtk_file_chooser_button_set_focus_on_click(p, to_gboolean(focus_on_click));
        }
    }
}

impl_drop!(FileChooserButton);
impl_TraitWidget!(FileChooserButton);

impl ::FileChooserTrait for FileChooserButton {}
