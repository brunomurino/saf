use saf::job_config_file::{Action, JobConfig};
use serde_json::json;

mod actions {
    use crate::ActionRunner;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
    pub struct MyTest1 {
        pub name: String,
        pub client: String,
    }

    impl ActionRunner for MyTest1 {
        fn run(&self) -> &str {
            println!("hello from MyAction");
            self.client.as_str()
        }
    }

    #[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
    pub struct MyTest2 {
        pub name: String,
        pub client: String,
    }

    impl ActionRunner for MyTest2 {
        fn run(&self) -> &str {
            println!("hello from MyAction2");
            self.client.as_str()
        }
    }
}
use actions::{MyTest1, MyTest2};

use enum_dispatch::enum_dispatch;
use serde::Deserialize;

#[enum_dispatch(ActionRunner)]
#[derive(Debug, Deserialize, PartialEq)]
pub enum ActionsEnum {
    MyTest1,
    MyTest2,
}

#[enum_dispatch]
pub trait ActionRunner {
    fn run(&self) -> &str;
}

#[test]
fn main_test() {
    let mut transfer_config = JobConfig {
        action: Action {
            name: "MyTest1".to_string(),
            args: json!({
                "name": "REPORT_TEST",
                "client": "FOO",
            }),
        },
    };
    let action_args = "{\"client\": \"overriden\"}".to_string();
    let action: ActionsEnum = transfer_config.action.load_action(&action_args);

    assert_eq!(
        ActionsEnum::MyTest1(MyTest1 {
            name: "REPORT_TEST".to_string(),
            client: "overriden".to_string(),
        }),
        action,
        "Client hasn't been overriden."
    );

    let res = action.run();
    assert_eq!(res, "overriden", "Action result is not what was expected");
}
