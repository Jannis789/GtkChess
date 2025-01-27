/* window.rs
 *
 * Copyright 2024 Jannis Rustige
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/GtkChess/gtk/window.ui")]
    pub struct GtkchessWindow {
        #[template_child]
        pub innerBox: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkchessWindow {
        const NAME: &'static str = "GtkChessWindow";
        type Type = super::GtkchessWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GtkchessWindow {}
    impl WidgetImpl for GtkchessWindow {}
    impl WindowImpl for GtkchessWindow {}
    impl ApplicationWindowImpl for GtkchessWindow {}
    impl AdwApplicationWindowImpl for GtkchessWindow {}
}

glib::wrapper! {
    pub struct GtkchessWindow(ObjectSubclass<imp::GtkchessWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow, @implements gio::ActionGroup, gio::ActionMap;
}

impl GtkchessWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}

