// Using the UserQueryableSource trait, we can implement it for String, Path, and GroupFile.
// By creating the base implementation for String,
// we can leverate that for the Path implementation,
// then use that Path implementation for the GroupFile implementation.
//
// If the process was much different for getting a user from a group file
// versus getting it from a web api or something,
// I could write the implementation for Path, and have its own code block,
// then also implement it for url::Url with its own code block.
//

use std::fs;
use std::path::Path;

pub trait UserQueryableSource {
    fn get_users_in_group(&self, group: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

impl UserQueryableSource for String {
    fn get_users_in_group(&self, group: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut users: Vec<String> = Vec::new();
        for line in self.lines() {
            if !line.starts_with(group) {
                continue
            }
            let mut results = line
                .split(":")
                .last()
                .unwrap_or_default()
                .split(",")
                .map(|u| u.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            users.append(&mut results)
        }
        Ok(users)
    }
}

impl UserQueryableSource for Path {
    fn get_users_in_group(&self, group: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        fs::read_to_string(self)?.get_users_in_group(group)
    }
}

pub struct GroupFile {
    pub path: String
}

impl UserQueryableSource for GroupFile {
    fn get_users_in_group(&self, group: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Path::new(&self.path).get_users_in_group(group)
    }
}

pub struct GetentCommand;

impl UserQueryableSource for GetentCommand {
    fn get_users_in_group(&self, group: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut command = std::process::Command::new("getent");
        command.args(&["group", group]);
        let command_output = command.output()?;
        let output_str = String::from_utf8(command_output.stdout)?;
        let users = output_str
            .split(':')
            .last()
            .expect("")
            .split(',')
            .map(|u| u.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(users)
    }
}
