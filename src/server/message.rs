use std::str;

pub enum Message {
    Text(String),
    ChangeName(String),
    Error(String),
}

impl Message {
    const COMMAND_FLAG: &'static str = "--";
    pub fn parse(data: String) -> Self {
        if data.contains(Self::COMMAND_FLAG) {
            let name = data
                .split(Self::COMMAND_FLAG)
                .map(|item| item.trim())
                .take(1)
                .last()
                .expect("Invalid command");
            if name.is_empty() {
                Self::Error("No arg".to_string())
            } else {
                Self::ChangeName(name.to_string())
            }
        } else {
            Self::Text(data)
        }
    }
}
