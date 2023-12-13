use crate::chamber::ChamberId;
use gtk::glib;
use gtk::glib::Object;

mod imp {
    use std::cell::Cell;
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    use crate::chamber::ChamberId;

    // Object holding the state
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ChamberObject)]
    pub struct ChamberObject {
        #[property(get, set)]
        valid: Cell<bool>,
        #[property(get, set)]
        chamber: Cell<ChamberId>,
        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChamberObject {
        const NAME: &'static str = "ChamberObject";
        type Type = super::ChamberObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ChamberObject {}
}

glib::wrapper! {
    pub struct ChamberObject(ObjectSubclass<imp::ChamberObject>);
}

impl ChamberObject {
    pub fn new(chamber: Option<ChamberId>, name: String) -> Self {
        Object::builder()
            .property("valid", chamber != None)
            .property("chamber", chamber.unwrap_or(0))
            .property("name", name)
            .build()
    }
}
