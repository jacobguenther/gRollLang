// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TreeIter(Boxed<gtk_sys::GtkTreeIter>);

    match fn {
        copy => |ptr| gtk_sys::gtk_tree_iter_copy(mut_override(ptr)),
        free => |ptr| gtk_sys::gtk_tree_iter_free(ptr),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || gtk_sys::gtk_tree_iter_get_type(),
    }
}
