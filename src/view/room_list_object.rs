use crate::room::RoomId;
use gtk::glib;
use gtk::glib::Object;

mod imp {
    use std::cell::Cell;
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    use crate::room::RoomId;

    // Object holding the state
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::RoomObject)]
    pub struct RoomObject {
        #[property(get, set)]
        valid: Cell<bool>,
        #[property(get, set)]
        room: Cell<RoomId>,
        #[property(get, set)]
        name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RoomObject {
        const NAME: &'static str = "RoomObject";
        type Type = super::RoomObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for RoomObject {}
}

glib::wrapper! {
    pub struct RoomObject(ObjectSubclass<imp::RoomObject>);
}

impl RoomObject {
    pub fn new(room: Option<RoomId>, name: String) -> Self {
        Object::builder()
            .property("valid", room != None)
            .property("room", room.unwrap_or(0))
            .property("name", name)
            .build()
    }
}
