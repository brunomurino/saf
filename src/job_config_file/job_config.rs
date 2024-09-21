use json_value_merge::Merge;
use minijinja::{Environment, Value};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JobConfig {
    pub action: Action,
}

impl JobConfig {
    pub fn load_job_config_file<T: serde::de::DeserializeOwned>(
        job_config_file: &PathBuf,
        jinja_ctx: serde_json::Value,
    ) -> T {
        println!("Loading transfer config file {:?}", job_config_file);

        let content = std::fs::read_to_string(job_config_file).expect("could not read file");

        let mut env = Environment::new();
        env.add_template("this", &content).unwrap();
        let tmpl = env.get_template("this").unwrap();
        let rendered_tmpl = tmpl.render(Value::from_serialize(jinja_ctx)).unwrap();

        let config: T = toml::from_str(&rendered_tmpl).unwrap();

        config
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Action {
    pub name: String,
    pub args: serde_json::Value,
}

impl Action {
    pub fn update_args(&mut self, new_args: &String) {
        let overriding_args = serde_json::from_str::<serde_json::Value>(new_args).unwrap();
        println!("Updating args with: {:#?}", overriding_args);
        self.args.merge(&overriding_args);
    }
    pub fn load_action<T: serde::de::DeserializeOwned>(&mut self, overriding_args: &String) -> T {
        self.update_args(overriding_args);
        let full_json = format!("{{{:?}: {}}}", self.name, self.args);
        let action: T = serde_json::from_str(&full_json).unwrap();
        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test1() {
        let my_transfer_path = PathBuf::from("tests/__test_jobs/test1.toml");

        let jinja_ctx = json!({"ENV": "TEST"});

        let transfer_config: JobConfig =
            JobConfig::load_job_config_file(&my_transfer_path, jinja_ctx);

        let expected_transfer_config = JobConfig {
            action: Action {
                name: "MyTest1".to_string(),
                args: json!({
                    "name": "REPORT_TEST",
                    "client": "FOO",
                }),
            },
        };

        assert_eq!(transfer_config, expected_transfer_config,)
    }
}
