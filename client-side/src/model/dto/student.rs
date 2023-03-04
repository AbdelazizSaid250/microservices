use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Student {
    pub(crate) name: String,
    pub(crate) gae: i8,
}

impl Student {
    pub(crate) fn new() -> Self {
        Self {
            name: "Abdelaziz".to_string(),
            gae: 28,
        }
    }
}