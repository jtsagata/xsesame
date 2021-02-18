// use glib::signal::Inhibit;
// use gtk::prelude::*;
// use relm::{Component, Widget};
// use relm_derive::{widget, Msg};
//
// #[widget]
// impl Widget for Win {
//   view! {
//         #[name="window"]
//         gtk::Window {
//             titlebar: Some(self.model.titlebar.widget()),
//             property_default_width: 1000,
//             property_default_height: 650,
//             #[name="main_window_stack"]
//             gtk::Stack {
//                 #[name="events"]
//                 EventView((self.model.config.clone(), self.model.accel_group.clone())) {
//                     child: {
//                         name: Some("events"),
//                         icon_name: Some("view-list-symbolic")
//                     },
//                 },
//                 #[name="event_sources"]
//                 EventSources(self.model.config.clone()) {
//                     child: {
//                         name: Some("event-sources"),
//                         icon_name: Some("document-properties-symbolic")
//                     },
//                 }
//             },
//             // Use a tuple when you want to both send a message and return a value to
//             // the GTK+ callback.
//             delete_event(_, _) => (Msg::Quit, Inhibit(false)),
//             key_press_event(_, key) => (Msg::KeyPress(key.clone()), Inhibit(false)),
//         }
//     }
// }
