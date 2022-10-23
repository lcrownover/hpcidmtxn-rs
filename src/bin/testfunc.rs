use std::process::Command;

struct GroupFile {path: String}

struct GetentGroup {}

trait UserQueryableSource {
    fn get_users(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

impl UserQueryableSource for GroupFile {
    fn get_users(&self, path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let command = Command::new("grep").args(&[pirg, &path]).output()?;
        let output_str = String::from_utf8(command.stdout)?;
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


fn get_users_in_group(group: &String, query_source: UserQueryCommand) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match query_source {
        UserQueryCommand::GroupFile(path) => {
            let command = Command::new("grep").args(&[pirg, &path]).output()?;
            let output_str = String::from_utf8(command.stdout)?;
            let users = output_str
                .split(':')
                .last()
                .expect("")
                .split(',')
                .map(|u| u.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Ok(users)
        },
        UserQueryCommand::GetentGroup => {
            let command = Command::new("getent").args(&["group", &group]).output()?;
            let output_str = String::from_utf8(command.stdout)?;
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
}

fn main() {
    users = get_users_in_group("docker", )
}
