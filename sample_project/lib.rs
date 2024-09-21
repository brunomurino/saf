mod actions;
use actions::MyAction;

use enum_dispatch::enum_dispatch;
use serde::Deserialize;

#[enum_dispatch(ActionRunner)]
#[derive(Debug, Deserialize)]
pub enum ActionsEnum {
    MyAction,
}

#[enum_dispatch]
pub trait ActionRunner {
    fn run(&self);
}
