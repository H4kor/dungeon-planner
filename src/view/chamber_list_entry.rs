use glib::Object;
use gtk::{
    glib,
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    Label,
};

use crate::chamber::{Chamber, ChamberId};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::subclass::prelude::*;
    use gtk::{glib, Label};

    use crate::chamber::ChamberId;

    // Object holding the state
    #[derive(Default)]
    pub struct ChamberListEntry {
        pub chamber_id: Cell<ChamberId>,
        pub label: RefCell<Label>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ChamberListEntry {
        const NAME: &'static str = "DungeonPlannerChamberListEntry";
        type Type = super::ChamberListEntry;
        type ParentType = gtk::ListBoxRow;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ChamberListEntry {}

    // Trait shared by all widgets
    impl WidgetImpl for ChamberListEntry {}

    // Trait shared by all labels
    impl ListBoxRowImpl for ChamberListEntry {}
}

glib::wrapper! {
    pub struct ChamberListEntry(ObjectSubclass<imp::ChamberListEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ChamberListEntry {
    fn chamber_to_label(chamber: &Chamber) -> String {
        format!("{}) {}", chamber.id.unwrap(), &chamber.name)
    }

    pub fn new(chamber: &Chamber) -> Self {
        let chamber_id = chamber.id.unwrap();
        let label = Label::new(Some(&ChamberListEntry::chamber_to_label(chamber)));
        label.set_xalign(0.01);
        let o: Self = Object::builder().property("child", label.clone()).build();
        let imp = o.imp();
        imp.chamber_id.set(chamber_id);
        imp.label.replace(label.clone());
        o
    }

    pub fn chamber_id(&self) -> ChamberId {
        let imp = imp::ChamberListEntry::from_obj(&self);
        imp.chamber_id.get()
    }

    pub fn update(&mut self, chamber: &Chamber) {
        self.imp()
            .label
            .borrow_mut()
            .set_label(&ChamberListEntry::chamber_to_label(chamber));
    }
}
