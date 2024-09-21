use serde_json::json;

use app::ActionRunner;
use app::ActionsEnum;
use saf::job_config_file::{find_job_config_file, TransferConfig};
use saf::App;

fn main() {
    let app = App::get();

    let my_transfer_path = find_job_config_file(&app.scope, &app.job_config_repo);

    let jinja_ctx = json!({});

    let mut transfer_config: TransferConfig =
        TransferConfig::load_job_config_file(&my_transfer_path, jinja_ctx);

    let action: ActionsEnum = transfer_config.action.load_action(&app.action_args);

    println!("{:#?}", action);

    action.run();
}
