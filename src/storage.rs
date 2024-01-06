use crate::chamber::{ChamberId, WallId};
use crate::common::Vec2;
use crate::door::{Door, DoorId};
use crate::object::{ObjectId, ObjectStyle};
use crate::state::{EditMode, StateCommand};
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;

fn line_to_command(l: &String) -> Option<StateCommand> {
    println!("{}", l);
    match l.split_once(" >> ") {
        None => None,
        Some((name, data)) => match name {
            "AddChamber" => Some(StateCommand::AddChamber),
            "SelectChamber" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::SelectChamber(
                    match v["chamber_id"].as_u64() {
                        Some(x) => Some(x as ChamberId),
                        None => None,
                    },
                ))
            }
            "SelectDoor" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::SelectDoor(match v["door_id"].as_u64() {
                    Some(x) => Some(x as ChamberId),
                    None => None,
                }))
            }
            "SelectObject" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::SelectObject(match v["object_id"].as_u64() {
                    Some(x) => Some(x as ChamberId),
                    None => None,
                }))
            }
            "AddVertexToChamber" => {
                println!("{}", data);
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::AddVertexToChamber(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                    Vec2 {
                        x: v["x"].as_i64().unwrap() as i32,
                        y: v["y"].as_i64().unwrap() as i32,
                    },
                ))
            }
            "ChangeChamberName" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeChamberName(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                    v["name"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeChamberNotes" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeChamberNotes(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                    v["notes"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeChamberHidden" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeChamberHidden(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                    v["hidden"].as_bool().unwrap(),
                ))
            }
            "ChangeMode" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeMode(EditMode::from_str(
                    v["mode"].as_str().unwrap(),
                )))
            }
            "SplitWall" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::SplitWall(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                    v["wall_id"].as_u64().unwrap() as WallId,
                    Vec2 {
                        x: v["x"].as_i64().unwrap() as i32,
                        y: v["y"].as_i64().unwrap() as i32,
                    },
                ))
            }
            "CollapseWall" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::CollapseWall(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                    v["wall_id"].as_u64().unwrap() as WallId,
                ))
            }
            "DeleteChamber" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::DeleteChamber(
                    v["chamber_id"].as_u64().unwrap() as ChamberId,
                ))
            }
            "AddDoor" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::AddDoor(Door::new(
                    v["part_of"].as_u64().unwrap() as ChamberId,
                    None,
                    v["width"].as_f64().unwrap(),
                    v["on_wall"].as_u64().unwrap() as ChamberId,
                    v["position"].as_f64().unwrap(),
                )))
            }
            "ChangeDoorName" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeDoorName(
                    v["door_id"].as_u64().unwrap() as DoorId,
                    v["name"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeDoorNotes" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeDoorNotes(
                    v["door_id"].as_u64().unwrap() as DoorId,
                    v["notes"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeDoorLeadsTo" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeDoorLeadsTo(
                    v["door_id"].as_u64().unwrap() as DoorId,
                    match v["chamber_id"].as_u64() {
                        Some(x) => Some(x as ChamberId),
                        None => None,
                    },
                ))
            }
            "ChangeDoorHidden" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeDoorHidden(
                    v["door_id"].as_u64().unwrap() as DoorId,
                    v["hidden"].as_bool().unwrap(),
                ))
            }
            "DeleteDoor" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::DeleteDoor(
                    v["door_id"].as_u64().unwrap() as DoorId
                ))
            }
            "AddObject" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::AddObject(
                    Vec2 {
                        x: v["x"].as_i64().unwrap() as i32,
                        y: v["y"].as_i64().unwrap() as i32,
                    },
                    match v["part_of"].as_u64() {
                        Some(i) => Some(i as u32),
                        None => None,
                    },
                ))
            }
            "ChangeDungeonName" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeDungeonName(
                    v["name"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeDungeonNotes" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeDungeonNotes(
                    v["notes"].as_str().unwrap().to_owned(),
                ))
            }
            "DeleteObject" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::DeleteObject(
                    v["object_id"].as_u64().unwrap() as ObjectId
                ))
            }
            "ChangeObjectName" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeObjectName(
                    v["object_id"].as_u64().unwrap() as ObjectId,
                    v["name"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeObjectNotes" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeObjectNotes(
                    v["object_id"].as_u64().unwrap() as ObjectId,
                    v["notes"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeObjectHidden" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeObjectHidden(
                    v["object_id"].as_u64().unwrap() as ObjectId,
                    v["hidden"].as_bool().unwrap(),
                ))
            }
            "ChangeObjectStyle" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeObjectStyle(
                    v["object_id"].as_u64().unwrap() as ObjectId,
                    ObjectStyle::from_str(v["style"].as_str().unwrap()),
                ))
            }

            _ => None,
        },
    }
}

pub fn load_dungeon(path: String) -> Vec<StateCommand> {
    if let Ok(_) = File::open(path.clone()) {
        let mut cmds = vec![];
        let lines: Vec<String> = read_to_string(path.clone())
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect(); // gather them together into a vector

        for line in lines {
            match line_to_command(&line) {
                None => {
                    println!("Unable to interpret line as command: {}", line);
                }
                Some(cmd) => cmds.push(cmd),
            };
        }
        cmds
    } else {
        vec![]
    }
}

pub fn save_to_file(save_file: String, cmds: &Vec<StateCommand>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .open(save_file.clone())
        .unwrap();

    let mut data_str = String::new();
    for cmd in cmds {
        let name = match cmd {
            StateCommand::AddChamber => "AddChamber".to_owned(),
            StateCommand::SelectChamber(_) => "SelectChamber".to_owned(),
            StateCommand::SelectDoor(_) => "SelectDoor".to_owned(),
            StateCommand::SelectObject(_) => "SelectObject".to_owned(),
            StateCommand::AddVertexToChamber(_, _) => "AddVertexToChamber".to_owned(),
            StateCommand::ChangeChamberName(_, _) => "ChangeChamberName".to_owned(),
            StateCommand::ChangeChamberNotes(_, _) => "ChangeChamberNotes".to_owned(),
            StateCommand::ChangeChamberHidden(_, _) => "ChangeChamberHidden".to_owned(),
            StateCommand::ChangeMode(_) => "ChangeMode".to_owned(),
            StateCommand::SplitWall(_, _, _) => "SplitWall".to_owned(),
            StateCommand::CollapseWall(_, _) => "CollapseWall".to_owned(),
            StateCommand::DeleteChamber(_) => "DeleteChamber".to_owned(),
            StateCommand::AddDoor(_) => "AddDoor".to_owned(),
            StateCommand::ChangeDoorName(_, _) => "ChangeDoorName".to_owned(),
            StateCommand::ChangeDoorNotes(_, _) => "ChangeDoorNotes".to_owned(),
            StateCommand::ChangeDoorLeadsTo(_, _) => "ChangeDoorLeadsTo".to_owned(),
            StateCommand::ChangeDoorHidden(_, _) => "ChangeDoorHidden".to_owned(),
            StateCommand::DeleteDoor(_) => "DeleteDoor".to_owned(),
            StateCommand::ChangeDungeonName(_) => "ChangeDungeonName".to_owned(),
            StateCommand::ChangeDungeonNotes(_) => "ChangeDungeonNotes".to_owned(),
            StateCommand::AddObject(_, _) => "AddObject".to_owned(),
            StateCommand::DeleteObject(_) => "DeleteObject".to_owned(),
            StateCommand::ChangeObjectName(_, _) => "ChangeObjectName".to_owned(),
            StateCommand::ChangeObjectNotes(_, _) => "ChangeObjectNotes".to_owned(),
            StateCommand::ChangeObjectHidden(_, _) => "ChangeObjectHidden".to_owned(),
            StateCommand::ChangeObjectStyle(_, _) => "ChangeObjectStyle".to_owned(),
        };
        let data = match cmd {
            StateCommand::AddChamber => serde_json::Value::Null,
            StateCommand::SelectChamber(chamber_id) => json!({ "chamber_id": chamber_id }),
            StateCommand::SelectDoor(door_id) => json!({ "door_id": door_id }),
            StateCommand::SelectObject(object_id) => json!({ "object_id": object_id }),
            StateCommand::AddVertexToChamber(chamber_id, pos) => json!({
                "chamber_id": chamber_id,
                "x": pos.x,
                "y": pos.y
            }),
            StateCommand::ChangeChamberName(chamber_id, name) => json!({
                "chamber_id": chamber_id,
                "name": name,
            }),
            StateCommand::ChangeChamberNotes(chamber_id, notes) => json!({
                "chamber_id": chamber_id,
                "notes": notes,
            }),
            StateCommand::ChangeChamberHidden(chamber_id, hidden) => json!({
                "chamber_id": chamber_id,
                "hidden": hidden,
            }),
            StateCommand::ChangeMode(mode) => json!({
                "mode": mode.to_str()
            }),
            StateCommand::SplitWall(chamber_id, wall_id, pos) => json!({
                "chamber_id": chamber_id,
                "wall_id": wall_id,
                "x": pos.x,
                "y": pos.y
            }),
            StateCommand::CollapseWall(chamber_id, wall_id) => json!({
                "chamber_id": chamber_id,
                "wall_id": wall_id,
            }),
            StateCommand::DeleteChamber(chamber_id) => json!({ "chamber_id": chamber_id }),
            StateCommand::AddDoor(door) => json!({
                "part_of": door.part_of,
                "width": door.width,
                "on_wall": door.on_wall,
                "position": door.position,
            }),
            StateCommand::ChangeDoorName(door_id, name) => json!({
                "door_id": door_id,
                "name": name,
            }),
            StateCommand::ChangeDoorNotes(door_id, notes) => json!({
                "door_id": door_id,
                "notes": notes,
            }),
            StateCommand::ChangeDoorLeadsTo(door_id, chamber_id) => json!({
                "door_id": door_id,
                "chamber_id": chamber_id,
            }),
            StateCommand::ChangeDoorHidden(door_id, hidden) => json!({
                "door_id": door_id,
                "hidden": hidden,
            }),
            StateCommand::DeleteDoor(door_id) => json!({ "door_id": door_id }),
            StateCommand::ChangeDungeonName(name) => json!({
                "name": name,
            }),
            StateCommand::ChangeDungeonNotes(notes) => json!({
                "notes": notes,
            }),
            StateCommand::AddObject(pos, part_of) => json!({
                "x": pos.x,
                "y": pos.y,
                "part_of": part_of,
            }),
            StateCommand::DeleteObject(object_id) => json!({ "object_id": object_id }),
            StateCommand::ChangeObjectName(object_id, name) => json!({
                "object_id": object_id,
                "name": name,
            }),
            StateCommand::ChangeObjectNotes(object_id, notes) => json!({
                "object_id": object_id,
                "notes": notes,
            }),
            StateCommand::ChangeObjectHidden(object_id, hidden) => json!({
                "object_id": object_id,
                "hidden": hidden,
            }),
            StateCommand::ChangeObjectStyle(object_id, style) => json!({
                "object_id": object_id,
                "style": style.to_str(),
            }),
        };
        data_str += format!("{} >> {}\n", name, data).as_str();
    }
    file.write(data_str.as_bytes()).unwrap();
    file.flush().unwrap();
}
