use std::{
    fmt::Display,
    io::{self, Write},
    process::Command,
    thread,
    time::{Duration, Instant},
};

use inquire::Select;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Issue {
    number: u64,
    title: String,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{} | {}", self.number, self.title))
    }
}

fn main() {
    let issue = Select::new("Select an issue to work on", get_issue_list().unwrap()).prompt().unwrap();

    println!("Starting work on {issue}...\n");

    timer().unwrap();
}

fn timer() -> io::Result<()> {
    let start_time = Instant::now();

    loop {
        let elapsed_time = Instant::now() - start_time;
        let elapsed_minutes = (elapsed_time.as_secs() / 60) as u64;
        let elapsed_seconds = elapsed_time.as_secs() - (elapsed_minutes * 60);

        print!("\r{:02}:{:02}", elapsed_minutes, elapsed_seconds);
        std::io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}

fn get_issue_list() -> anyhow::Result<Vec<Issue>> {
    let mut assigned_issues: Vec<Issue> = serde_json::from_str(&run_gh_command(
        "cs481-ekh/s25-sprout-squad",
        AssigneeVariant::CurrentUser,
    )?)?;
    let mut unassigned_issues: Vec<Issue> = serde_json::from_str(&run_gh_command(
        "cs481-ekh/s25-sprout-squad",
        AssigneeVariant::None,
    )?)?;

    assigned_issues.sort();
    unassigned_issues.sort();

    assigned_issues.append(&mut unassigned_issues);

    Ok(assigned_issues)
}

fn run_gh_command(repo: &str, assignee: AssigneeVariant) -> anyhow::Result<String> {
    let mut cmd = Command::new("gh");

    cmd.args([
        "issue",
        "list",
        "--label",
        "task",
        "--json",
        "number,title",
        "--repo",
    ])
    .arg(repo)
    .arg("--search")
    .arg(assignee.to_string());

    let result = cmd.output()?;
    if !result.status.success() {
        return Err(anyhow::anyhow!(
            "received exit status {} while running gh.\n{}\n{}",
            result.status,
            String::from_utf8(result.stdout)?,
            String::from_utf8(result.stderr)?
        ));
    }

    Ok(String::from_utf8(result.stdout)?)
}

enum AssigneeVariant {
    None,
    CurrentUser,
//    User(String),
}

impl Display for AssigneeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                AssigneeVariant::None => "no:assignee".to_owned(),
                AssigneeVariant::CurrentUser => "assignee:@me".to_owned(),
 //               AssigneeVariant::User(user) => "assignee".to_owned() + user,
            }
        ))
    }
}
