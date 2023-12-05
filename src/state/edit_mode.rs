#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditMode {
    Select,
    AppendRoom,
    SplitEdge,
    AddDoor,
}

impl EditMode {
    pub fn to_str(&self) -> String {
        match self {
            EditMode::Select => "Select".to_owned(),
            EditMode::AppendRoom => "AppendRoom".to_owned(),
            EditMode::SplitEdge => "SplitEdge".to_owned(),
            EditMode::AddDoor => "AddDoor".to_owned(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Select" => EditMode::Select,
            "AppendRoom" => EditMode::AppendRoom,
            "SplitEdge" => EditMode::SplitEdge,
            "AddDoor" => EditMode::AddDoor,
            _ => todo!(),
        }
    }
}
