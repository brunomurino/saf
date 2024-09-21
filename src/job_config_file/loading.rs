use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_job_config_file(name: &str, repo: &Path) -> PathBuf {
    println!("Looking for Scope named \"{}\"", &name);
    println!("Using Job Config Repo {:?}", &repo);
    let result: Result<Vec<DirEntry>, walkdir::Error> = WalkDir::new(repo)
        .into_iter()
        .filter(|x: &Result<DirEntry, walkdir::Error>| {
            x.as_ref()
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                == name
        })
        .collect();

    let result_ok = result.unwrap();

    if result_ok.len() == 0 {
        panic!("Failed to find Job Config with name `{}`", name)
    }
    if result_ok.len() > 1 {
        panic!("Found more than 1 Job Config with name `{}`", name)
    }

    let my_transfer_path: PathBuf = result_ok[0].clone().path().to_owned();
    println!("Found Job Config file `{:#?}`", &my_transfer_path);

    my_transfer_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_job_config_file_test1() {
        let scope = "test1";
        let job_config_repo = PathBuf::from("tests/__test_jobs");
        let my_transfer_path = find_job_config_file(&scope, &job_config_repo);
        assert_eq!(
            my_transfer_path,
            PathBuf::from("tests/__test_jobs/test1.toml")
        )
    }

    #[test]
    fn test_find_job_config_file_test2() {
        let scope = "test2";
        let job_config_repo = PathBuf::from("tests/__test_jobs");
        let my_transfer_path = find_job_config_file(&scope, &job_config_repo);
        assert_eq!(
            my_transfer_path,
            PathBuf::from("tests/__test_jobs/subfo/test2.toml")
        )
    }
}
