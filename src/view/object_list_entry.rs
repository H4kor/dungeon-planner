use glib::Object;
use gtk::{
    glib,
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    Label,
};

use crate::object::{Object as DungeonObject, ObjectId};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::subclass::prelude::*;
    use gtk::{glib, Label};

    use crate::object::ObjectId;

    // Object holding the state
    #[derive(Default)]
    pub struct ObjectListEntry {
        pub object_id: Cell<ObjectId>,
        pub label: RefCell<Label>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ObjectListEntry {
        const NAME: &'static str = "DungeonPlannerObjectListEntry";
        type Type = super::ObjectListEntry;
        type ParentType = gtk::ListBoxRow;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ObjectListEntry {}

    // Trait shared by all widgets
    impl WidgetImpl for ObjectListEntry {}

    // Trait shared by all labels
    impl ListBoxRowImpl for ObjectListEntry {}
}

glib::wrapper! {
    pub struct ObjectListEntry(ObjectSubclass<imp::ObjectListEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ObjectListEntry {
    fn object_to_label(object: &DungeonObject) -> String {
        if object.name.is_empty() {
            format!("{}) {}", object.id, "Object")
        } else {
            format!("{}) {}", object.id, &object.name)
        }
    }

    pub fn new(object: &DungeonObject) -> Self {
        let object_id = object.id;

        let label = Label::new(Some(&ObjectListEntry::object_to_label(object)));
        label.set_xalign(0.01);
        let o: Self = Object::builder().property("child", label.clone()).build();
        let imp = o.imp();
        imp.object_id.set(object_id);
        imp.label.replace(label.clone());
        o
    }

    pub fn object_id(&self) -> ObjectId {
        let imp = imp::ObjectListEntry::from_obj(&self);
        imp.object_id.get()
    }

    pub fn update(&mut self, object: &DungeonObject) {
        let name = ObjectListEntry::object_to_label(object);
        self.imp().label.borrow_mut().set_label(&name);
    }
}
