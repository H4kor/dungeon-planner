use crate::{
    chamber::{Chamber, ChamberId, Wall},
    common::Vec2,
    door::{Door, DoorId},
    object::{Object, ObjectId},
};

/// A Dungeon is the main object we care about
/// It consists of multiple chambers
pub struct Dungeon {
    pub chambers: Vec<Chamber>,
    pub doors: Vec<Door>,
    pub objects: Vec<Object>,
    pub name: String,
    pub notes: String,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            chambers: vec![],
            doors: vec![],
            objects: vec![],
            name: "".to_owned(),
            notes: "".to_owned(),
        }
    }

    /// Add a Chamber to the dungeon
    /// If the Chamber does not have an id yet, it is generate before insertion
    ///
    /// Returns the `ChamberId`
    pub fn add_chamber(&mut self, mut chamber: Chamber) -> ChamberId {
        // if Id is already used, generate a new one.
        let chamber_id = match self.chamber_mut(chamber.id) {
            Some(_) => self.next_chamber_id(),
            None => chamber.id,
        };
        chamber.id = chamber_id;
        self.chambers.push(chamber);
        chamber_id
    }

    /// generates an unused `ChamberId`.
    fn next_chamber_id(&self) -> ChamberId {
        let max_id = self.chambers.iter().map(|r| r.id).max();
        match max_id {
            None => 1,
            Some(x) => x + 1,
        }
    }

    /// get a chamber by its id
    pub fn chamber_mut(&mut self, chamber_id: ChamberId) -> Option<&mut Chamber> {
        self.chambers.iter_mut().find(|r| r.id == chamber_id)
    }
    pub fn chamber(&self, chamber_id: ChamberId) -> Option<&Chamber> {
        self.chambers.iter().find(|r| r.id == chamber_id)
    }

    /// get the nearest wall to a given point
    /// Returns the ChamberId and WallId
    pub fn nearest_wall(&self, pos: Vec2<f64>) -> Option<(ChamberId, Wall)> {
        let mut min_chamber_id = None;
        let mut min_wall = None;
        let mut min_d = f64::INFINITY;
        for chamber in self.chambers.iter() {
            if let Some(wall) = chamber.nearest_wall(pos) {
                let d = wall.distance(pos);
                if d < min_d {
                    min_chamber_id = Some(chamber.id);
                    min_wall = Some(wall);
                    min_d = d;
                }
            }
        }
        match min_chamber_id {
            None => None,
            Some(chamber_id) => Some((chamber_id, min_wall.unwrap())),
        }
    }

    pub(crate) fn chamber_at(&self, pos: Vec2<f64>) -> Option<ChamberId> {
        for chamber in &self.chambers {
            if chamber.contains_point(pos.into()) {
                return Some(chamber.id);
            }
        }
        None
    }

    pub(crate) fn chambers(&self) -> &Vec<Chamber> {
        &self.chambers
    }

    /**
     * Removes a chamber from the dungeon and all doors which are part of this chamber.
     * Returns a list of removed DoorIds
     */
    pub(crate) fn remove_chamber(&mut self, chamber_id: u32) -> Vec<DoorId> {
        let idx = self.chambers.iter().position(|r| r.id == chamber_id);
        match idx {
            Some(i) => {
                // remove all doors being part of this chamber first
                let door_ids = self
                    .doors
                    .iter()
                    .filter(|d| d.part_of == chamber_id)
                    .map(|d| d.id)
                    .collect();
                self.doors.retain(|d| d.part_of != chamber_id);
                self.chambers.remove(i);
                door_ids
            }
            None => {
                println!("Chamber Id not found for deletion");
                vec![]
            }
        }
    }

    pub fn door(&self, id: DoorId) -> Option<&Door> {
        self.doors.iter().find(|d| d.id == id)
    }

    pub fn door_mut(&mut self, id: DoorId) -> Option<&mut Door> {
        self.doors.iter_mut().find(|d| d.id == id)
    }

    pub(crate) fn door_at(&self, pos: Vec2<f64>) -> Option<DoorId> {
        for door in &self.doors {
            if door.contains_point(
                self.chamber(door.part_of)
                    .unwrap()
                    .wall(door.on_wall)
                    .unwrap(),
                pos.into(),
            ) {
                return Some(door.id);
            }
        }
        None
    }

    pub fn add_door(&mut self, mut door: Door) -> DoorId {
        // if Id is already used, generate a new one.
        let door_id = match self.door(door.id) {
            Some(_) => self.next_door_id(),
            None => door.id,
        };
        door.id = door_id;
        self.doors.push(door);
        door_id
    }

    fn next_door_id(&self) -> DoorId {
        self.doors.iter().map(|r| r.id).max().unwrap_or(0) + 1
    }

    pub(crate) fn chamber_doors(&self, chamber_id: ChamberId) -> Vec<&Door> {
        self.doors
            .iter()
            .filter(|d| d.part_of == chamber_id || d.leads_to == Some(chamber_id))
            .collect()
    }

    pub(crate) fn remove_door(&mut self, door_id: DoorId) {
        let idx = self.doors.iter().position(|r| r.id == door_id);
        match idx {
            Some(i) => {
                self.doors.remove(i);
            }
            None => {
                println!("door Id not found for deletion")
            }
        };
    }

    pub fn add_object(&mut self, mut obj: Object) -> ObjectId {
        let id = self.next_object_id();
        obj.id = id;
        self.objects.push(obj);
        id
    }

    fn next_object_id(&self) -> ObjectId {
        self.objects.iter().map(|r| r.id).max().unwrap_or(0) + 1
    }

    pub fn walls(&self) -> Vec<Wall> {
        let mut all_walls = Vec::<Wall>::new();
        for chamber in self.chambers.iter() {
            for wall in chamber.walls() {
                all_walls.push(wall.clone())
            }
        }
        all_walls
    }

    pub(crate) fn object_at(&self, pos: Vec2<f64>) -> Option<ObjectId> {
        for object in self.objects.iter() {
            if object.contains(pos) {
                return Some(object.id);
            }
        }
        None
    }

    pub(crate) fn object(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|o| o.id == id)
    }
}
