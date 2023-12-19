use crate::chamber::{ChamberId, WallId};
use crate::common::Vec2;
use crate::door::{Door, DoorId};
use crate::state::{EditMode, StateCommand, StateController};
use serde_json::json;
use serde_json::Value;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;
use std::{cell::RefCell, fs::File, rc::Rc};

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
            "DeleteChamber" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::DeleteChamber(
                    v["chamber_id"].as_u64().unwrap() as u32,
                ))
            }
            "AddDoor" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::AddDoor(Door::new(
                    v["part_of"].as_u64().unwrap() as u32,
                    None,
                    v["width"].as_f64().unwrap(),
                    v["on_wall"].as_u64().unwrap() as u32,
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
            "DeleteDoor" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::DeleteDoor(
                    v["door_id"].as_u64().unwrap() as u32
                ))
            }
            _ => None,
        },
    }
}

pub fn load_dungeon(control: Rc<RefCell<StateController>>, path: String) -> bool {
    if let Ok(_) = File::open(path.clone()) {
        let lines: Vec<String> = read_to_string(path.clone())
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect(); // gather them together into a vector

        for line in lines {
            match line_to_command(&line) {
                None => {
                    println!("Unable to interpret line as command: {}", line);
                    return false;
                }
                Some(cmd) => control.borrow_mut().apply(cmd),
            };
        }
        true
    } else {
        false
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
            StateCommand::AddVertexToChamber(_, _) => "AddVertexToChamber".to_owned(),
            StateCommand::ChangeChamberName(_, _) => "ChangeChamberName".to_owned(),
            StateCommand::ChangeChamberNotes(_, _) => "ChangeChamberNotes".to_owned(),
            StateCommand::ChangeChamberHidden(_, _) => "ChangeChamberHidden".to_owned(),
            StateCommand::ChangeMode(_) => "ChangeMode".to_owned(),
            StateCommand::SplitWall(_, _, _) => "SplitWall".to_owned(),
            StateCommand::DeleteChamber(_) => "DeleteChamber".to_owned(),
            StateCommand::AddDoor(_) => "AddDoor".to_owned(),
            StateCommand::ChangeDoorName(_, _) => "ChangeDoorName".to_owned(),
            StateCommand::ChangeDoorNotes(_, _) => "ChangeDoorNotes".to_owned(),
            StateCommand::ChangeDoorLeadsTo(_, _) => "ChangeDoorLeadsTo".to_owned(),
            StateCommand::DeleteDoor(_) => "DeleteDoor".to_owned(),
        };
        let data = match cmd {
            StateCommand::AddChamber => serde_json::Value::Null,
            StateCommand::SelectChamber(chamber_id) => json!({ "chamber_id": chamber_id }),
            StateCommand::SelectDoor(door_id) => json!({ "door_id": door_id }),
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
            StateCommand::DeleteDoor(door_id) => json!({ "door_id": door_id }),
        };
        data_str += format!("{} >> {}\n", name, data).as_str();
    }
    file.write(data_str.as_bytes()).unwrap();
    file.flush().unwrap();
}
