use serde_json::Value;

use crate::common::Vec2;
use crate::room::{Room, RoomId};
use crate::state::commands::menu::SelectRoomCommand;
use crate::state::commands::{
    AddRoomCommand, AddVertexToRoomCommand, ChangeRoomName, ChangeRoomNotes,
};
use crate::state::{StateCommand, StateController};
use std::io::{self, BufRead};
use std::{cell::RefCell, fs::File, rc::Rc};

fn line_to_command(l: &String) -> Option<RefCell<Box<dyn StateCommand>>> {
    match l.split_once(" >> ") {
        None => None,
        Some((name, data)) => match name {
            "AddRoomCommand" => Some(RefCell::new(Box::new(AddRoomCommand {}))),
            "SelectRoomCommand" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(RefCell::new(Box::new(SelectRoomCommand {
                    room_id: match v["room_id"].as_u64() {
                        Some(x) => Some(x as RoomId),
                        None => None,
                    },
                })))
            }
            "AddVertexToRoomCommand" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(RefCell::new(Box::new(AddVertexToRoomCommand {
                    room_id: v["room_id"].as_u64().unwrap() as RoomId,
                    pos: Vec2 {
                        x: v["x"].as_i64().unwrap() as i32,
                        y: v["y"].as_i64().unwrap() as i32,
                    },
                })))
            }
            "ChangeRoomName" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(RefCell::new(Box::new(ChangeRoomName {
                    room_id: v["room_id"].as_u64().unwrap() as RoomId,
                    name: v["name"].as_str().unwrap().to_owned(),
                })))
            }
            "ChangeRoomNotes" => {
                let v: Value = serde_json::from_str(data).unwrap();
                Some(RefCell::new(Box::new(ChangeRoomNotes {
                    room_id: v["room_id"].as_u64().unwrap() as RoomId,
                    notes: v["notes"].as_str().unwrap().to_owned(),
                })))
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
