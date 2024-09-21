use crate::ActionRunner;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MyAction {
    pub name: String,
    pub client: String,
}

impl ActionRunner for MyAction {
    fn run(&self) {
        println!("hello from MyAction")
    }
}
