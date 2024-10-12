use serde_json::json;

mod app {
    mod actions {
        use crate::ActionRunner;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct MyAction {
            pub name: String,
            pub client: String,
        }

        impl ActionRunner for MyAction {
            fn run(&self) -> String {
                println!("hello from MyAction");
                format!("{}@{}", self.name, self.client)
            }
        }

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct MyOtherAction {
            pub name: String,
            pub client: String,
        }

        impl ActionRunner for MyOtherAction {
            fn run(&self) -> String {
                println!("hello from MyOtherAction");
                format!("{}@{}", self.name, self.client)
            }
        }
    }

    use actions::{MyAction, MyOtherAction};

    use enum_dispatch::enum_dispatch;
    use serde::Deserialize;

    #[enum_dispatch(ActionRunner)]
    #[derive(Debug, Deserialize)]
    pub enum ActionsEnum {
        MyAction,
        MyOtherAction,
    }

    #[enum_dispatch]
    pub trait ActionRunner {
        fn run(&self) -> String;
    }
}

use app::ActionRunner;
use app::ActionsEnum;
use saf::job_config_file::{find_job_config_file, JobConfig};
use saf::App;

fn main() {
    let app = App::get();

    let my_transfer_path = find_job_config_file(&app.scope, &app.job_config_repo);

    let jinja_ctx = json!({"ENV": "PROD"});

    let mut transfer_config: JobConfig =
        JobConfig::load_job_config_file(&my_transfer_path, jinja_ctx);

    let action: ActionsEnum = transfer_config.action.load_action(&app.action_args);

    println!("{:#?}", action);

    let res = action.run();
    println!("{:#?}", res);
}
