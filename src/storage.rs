use crate::common::Vec2;
use crate::room::RoomId;
use crate::state::{EditMode, StateCommand, StateController};
use serde_json::json;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::{cell::RefCell, fs::File, rc::Rc};

fn line_to_command(l: &String) -> Option<StateCommand> {
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
            "AddVertexToRoomCommand" => {
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
            _ => None,
        },
    }
}

pub fn load_dungeon(control: Rc<RefCell<StateController>>) {
    if let Ok(file) = File::open("dungeon.txt") {
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(ip) = line {
                match line_to_command(&ip) {
                    None => {
                        println!("Unable to interpret line as command: {}", ip)
                    }
                    Some(cmd) => control.borrow_mut().apply(cmd),
                };
            }
        }
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
            StateCommand::AddVertexToRoom(_, _) => "AddVertexToRoomCommand".to_owned(),
            StateCommand::ChangeRoomName(_, _) => "ChangeRoomName".to_owned(),
            StateCommand::ChangeRoomNotes(_, _) => "ChangeRoomNotes".to_owned(),
            StateCommand::ChangeMode(_) => "ChangeMode".to_owned(),
        };
        let data = match cmd {
            StateCommand::AddRoom => serde_json::Value::Null,
            StateCommand::SelectRoom(room_id) => json!({"room_id": room_id}),
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
        };
        data_str += format!("{} >> {}\n", name, data).as_str();
    }
    file.write(data_str.as_bytes()).unwrap();
    file.flush().unwrap();
}
