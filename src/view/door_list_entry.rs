use glib::Object;
use gtk::{
    glib,
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    Label,
};

use crate::door::{Door, DoorId};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::subclass::prelude::*;
    use gtk::{glib, Label};

    use crate::door::DoorId;

    // Object holding the state
    #[derive(Default)]
    pub struct DoorListEntry {
        pub door_id: Cell<DoorId>,
        pub label: RefCell<Label>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for DoorListEntry {
        const NAME: &'static str = "DungeonPlannerDoorListEntry";
        type Type = super::DoorListEntry;
        type ParentType = gtk::ListBoxRow;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for DoorListEntry {}

    // Trait shared by all widgets
    impl WidgetImpl for DoorListEntry {}

    // Trait shared by all labels
    impl ListBoxRowImpl for DoorListEntry {}
}

glib::wrapper! {
    pub struct DoorListEntry(ObjectSubclass<imp::DoorListEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DoorListEntry {
    fn chamber_to_label(door: &Door) -> String {
        if door.name.is_empty() {
            format!("{}) {}", door.id, "Door")
        } else {
            format!("{}) {}", door.id, &door.name)
        }
    }

    pub fn new(door: &Door) -> Self {
        let door_id = door.id;

        let label = Label::new(Some(&DoorListEntry::chamber_to_label(door)));
        label.set_xalign(0.01);
        let o: Self = Object::builder().property("child", label.clone()).build();
        let imp = o.imp();
        imp.door_id.set(door_id);
        imp.label.replace(label.clone());
        o
    }

    pub fn door_id(&self) -> DoorId {
        let imp = imp::DoorListEntry::from_obj(&self);
        imp.door_id.get()
    }

    pub fn update(&mut self, door: &Door) {
        let name = DoorListEntry::chamber_to_label(door);
        self.imp().label.borrow_mut().set_label(&name);
    }
}
