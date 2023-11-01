use glib::Object;
use gtk::{
    glib,
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    Label,
};

use crate::room::{Room, RoomId};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::subclass::prelude::*;
    use gtk::{glib, Label};

    use crate::room::RoomId;

    // Object holding the state
    #[derive(Default)]
    pub struct RoomListEntry {
        pub room_id: Cell<RoomId>,
        pub label: RefCell<Label>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RoomListEntry {
        const NAME: &'static str = "DungeonPlannerRoomListEntry";
        type Type = super::RoomListEntry;
        type ParentType = gtk::ListBoxRow;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for RoomListEntry {}

    // Trait shared by all widgets
    impl WidgetImpl for RoomListEntry {}

    // Trait shared by all labels
    impl ListBoxRowImpl for RoomListEntry {}
}

glib::wrapper! {
    pub struct RoomListEntry(ObjectSubclass<imp::RoomListEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RoomListEntry {
    pub fn new(room: &Room) -> Self {
        let room_id = room.id.unwrap();
        let label = Label::new(Some(&room.name));
        let o: Self = Object::builder().property("child", label.clone()).build();
        let imp = o.imp();
        imp.room_id.set(room_id);
        imp.label.replace(label.clone());
        o
    }

    pub fn room_id(&self) -> RoomId {
        let imp = imp::RoomListEntry::from_obj(&self);
        imp.room_id.get()
    }

    pub fn update(&mut self, room: &Room) {
        self.imp().label.borrow_mut().set_label(&room.name);
    }
}
