use serde_json::Value;

use crate::common::Vec2;
use crate::room::RoomId;
use crate::state::{StateCommand, StateController};
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
                Some(StateCommand::ChangeRoomNodes(
                    v["room_id"].as_u64().unwrap() as RoomId,
                    v["notes"].as_str().unwrap().to_owned(),
                ))
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
