use crate::{
    chamber::{Chamber, ChamberId, Wall},
    common::Vec2,
    door::{Door, DoorId},
};

/// A Dungeon is the main object we care about
/// It consists of multiple chambers
pub struct Dungeon {
    pub chambers: Vec<Chamber>,
    pub doors: Vec<Door>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            chambers: vec![],
            doors: vec![],
        }
    }

    /// Add a Chamber to the dungeon
    /// If the Chamber does not have an id yet, it is generate before insertion
    ///
    /// Returns the `ChamberId`
    pub fn add_chamber(&mut self, mut chamber: Chamber) -> ChamberId {
        let chamber_id = match chamber.id {
            None => self.next_chamber_id(),
            Some(x) => {
                // if Id is already used, generate a new one.
                match self.chamber_mut(x) {
                    Some(_) => self.next_chamber_id(),
                    None => x,
                }
            }
        };
        chamber.id = Some(chamber_id);
        self.chambers.push(chamber);
        chamber_id
    }

    /// generates an unused `ChamberId`.
    fn next_chamber_id(&self) -> ChamberId {
        let max_id = self.chambers.iter().map(|r| r.id.unwrap_or(0)).max();
        match max_id {
            None => 1,
            Some(x) => x + 1,
        }
    }

    /// get a chamber by its id
    pub fn chamber_mut(&mut self, chamber_id: ChamberId) -> Option<&mut Chamber> {
        self.chambers.iter_mut().find(|r| r.id == Some(chamber_id))
    }
    pub fn chamber(&self, chamber_id: ChamberId) -> Option<&Chamber> {
        self.chambers.iter().find(|r| r.id == Some(chamber_id))
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
                    min_chamber_id = chamber.id;
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
                return chamber.id;
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
        let idx = self.chambers.iter().position(|r| r.id == Some(chamber_id));
        match idx {
            Some(i) => {
                // remove all doors being part of this chamber first
                let door_ids = self
                    .doors
                    .iter()
                    .filter(|d| d.part_of == chamber_id)
                    .map(|d| d.id.unwrap())
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
        self.doors.iter().find(|d| d.id == Some(id))
    }

    pub fn door_mut(&mut self, id: DoorId) -> Option<&mut Door> {
        self.doors.iter_mut().find(|d| d.id == Some(id))
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
                return door.id;
            }
        }
        None
    }

    pub fn add_door(&mut self, mut door: Door) -> DoorId {
        let door_id = match door.id {
            None => self.next_door_id(),
            Some(x) => {
                // if Id is already used, generate a new one.
                match self.door(x) {
                    Some(_) => self.next_door_id(),
                    None => x,
                }
            }
        };
        door.id = Some(door_id);
        self.doors.push(door);
        door_id
    }

    fn next_door_id(&self) -> DoorId {
        self.doors
            .iter()
            .map(|r| r.id.unwrap_or(0))
            .max()
            .unwrap_or(0)
            + 1
    }

    pub(crate) fn chamber_doors(&self, chamber_id: ChamberId) -> Vec<&Door> {
        self.doors
            .iter()
            .filter(|d| d.part_of == chamber_id || d.leads_to == Some(chamber_id))
            .collect()
    }

    pub(crate) fn remove_door(&mut self, door_id: DoorId) {
        let idx = self.doors.iter().position(|r| r.id == Some(door_id));
        match idx {
            Some(i) => {
                self.doors.remove(i);
            }
            None => {
                println!("door Id not found for deletion")
            }
        };
    }
}
