use crate::common::Vec2;
use crate::door::{Door, DoorId};
use crate::room::{RoomId, WallId};
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
            "AddRoomCommand" => Some(StateCommand::AddRoom),
            "SelectRoomCommand" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::SelectRoom(match v["room_id"].as_u64() {
                    Some(x) => Some(x as RoomId),
                    None => None,
                }))
            }
            "SelectDoor" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::SelectDoor(match v["door_id"].as_u64() {
                    Some(x) => Some(x as RoomId),
                    None => None,
                }))
            }
            "AddVertexToRoomCommand" => {
                println!("{}", data);
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::AddVertexToRoom(
                    v["room_id"].as_u64().unwrap() as RoomId,
                    Vec2 {
                        x: v["x"].as_i64().unwrap() as i32,
                        y: v["y"].as_i64().unwrap() as i32,
                    },
                ))
            }
            "ChangeRoomName" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeRoomName(
                    v["room_id"].as_u64().unwrap() as RoomId,
                    v["name"].as_str().unwrap().to_owned(),
                ))
            }
            "ChangeRoomNotes" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::ChangeRoomNotes(
                    v["room_id"].as_u64().unwrap() as RoomId,
                    v["notes"].as_str().unwrap().to_owned(),
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
                    v["room_id"].as_u64().unwrap() as RoomId,
                    v["wall_id"].as_u64().unwrap() as WallId,
                    Vec2 {
                        x: v["x"].as_i64().unwrap() as i32,
                        y: v["y"].as_i64().unwrap() as i32,
                    },
                ))
            }
            "DeleteRoom" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(StateCommand::DeleteRoom(
                    v["room_id"].as_u64().unwrap() as u32
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
                    match v["room_id"].as_u64() {
                        Some(x) => Some(x as RoomId),
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
            StateCommand::AddRoom => "AddRoomCommand".to_owned(),
            StateCommand::SelectRoom(_) => "SelectRoomCommand".to_owned(),
            StateCommand::SelectDoor(_) => "SelectDoor".to_owned(),
            StateCommand::AddVertexToRoom(_, _) => "AddVertexToRoomCommand".to_owned(),
            StateCommand::ChangeRoomName(_, _) => "ChangeRoomName".to_owned(),
            StateCommand::ChangeRoomNotes(_, _) => "ChangeRoomNotes".to_owned(),
            StateCommand::ChangeMode(_) => "ChangeMode".to_owned(),
            StateCommand::SplitWall(_, _, _) => "SplitWall".to_owned(),
            StateCommand::DeleteRoom(_) => "DeleteRoom".to_owned(),
            StateCommand::AddDoor(_) => "AddDoor".to_owned(),
            StateCommand::ChangeDoorName(_, _) => "ChangeDoorName".to_owned(),
            StateCommand::ChangeDoorNotes(_, _) => "ChangeDoorNotes".to_owned(),
            StateCommand::ChangeDoorLeadsTo(_, _) => "ChangeDoorLeadsTo".to_owned(),
            StateCommand::DeleteDoor(_) => "DeleteDoor".to_owned(),
        };
        let data = match cmd {
            StateCommand::AddRoom => serde_json::Value::Null,
            StateCommand::SelectRoom(room_id) => json!({ "room_id": room_id }),
            StateCommand::SelectDoor(door_id) => json!({ "door_id": door_id }),
            StateCommand::AddVertexToRoom(room_id, pos) => json!({
                "room_id": room_id,
                "x": pos.x,
                "y": pos.y
            }),
            StateCommand::ChangeRoomName(room_id, name) => json!({
                "room_id": room_id,
                "name": name,
            }),
            StateCommand::ChangeRoomNotes(room_id, notes) => json!({
                "room_id": room_id,
                "notes": notes,
            }),
            StateCommand::ChangeMode(mode) => json!({
                "mode": mode.to_str()
            }),
            StateCommand::SplitWall(room_id, wall_id, pos) => json!({
                "room_id": room_id,
                "wall_id": wall_id,
                "x": pos.x,
                "y": pos.y
            }),
            StateCommand::DeleteRoom(room_id) => json!({ "room_id": room_id }),
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
            StateCommand::ChangeDoorLeadsTo(door_id, room_id) => json!({
                "door_id": door_id,
                "room_id": room_id,
            }),
            StateCommand::DeleteDoor(door_id) => json!({ "door_id": door_id }),
        };
        data_str += format!("{} >> {}\n", name, data).as_str();
    }
    file.write(data_str.as_bytes()).unwrap();
    file.flush().unwrap();
}
