// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use glib::translate::ToGlibPtr;
use ffi;
use cast::GTK_WINDOW;
use FFIWidget;
use DialogButtons;

struct_Widget!(Dialog);

impl Dialog {
    pub fn new() -> Dialog {
        unsafe {
            ::FFIWidget::wrap_widget(
                ffi::gtk_dialog_new())
        }
    }

    pub fn with_buttons<T: DialogButtons>(title: &str, parent: Option<&::Window>,
                                          flags: ::DialogFlags, buttons: T) -> Dialog {
        unsafe {
            let parent = match parent {
                Some(w) => GTK_WINDOW(w.unwrap_widget()),
                None => ptr::null_mut(),
            };
            ::FFIWidget::wrap_widget(
                buttons.invoke3(ffi::gtk_dialog_new_with_buttons,
                                title.to_glib_none().0,
                                parent,
                                flags))
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::GtkWidget {
        self.pointer
    }
}

impl_drop!(Dialog);
impl_TraitWidget!(Dialog);

impl ::ContainerTrait for Dialog {}
impl ::BinTrait for Dialog {}
impl ::WindowTrait for Dialog {}
impl ::DialogTrait for Dialog {}
