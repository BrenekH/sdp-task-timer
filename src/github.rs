use std::{fmt::Display, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Issue {
    pub number: u64,
    pub title: String,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{} | {}", self.number, self.title))
    }
}

enum Assignee {
    None,
    CurrentUser,
    // User(String),
}

impl Display for Assignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Assignee::None => "no:assignee".to_owned(),
                Assignee::CurrentUser => "assignee:@me".to_owned(),
                // AssigneeVariant::User(user) => "assignee:".to_owned() + user,
            }
        ))
    }
}

pub fn get_issue_list(repo: &str, show_all: bool) -> anyhow::Result<Vec<Issue>> {
    let mut assigned_issues: Vec<Issue> =
        serde_json::from_str(&run_gh_issue_list(repo, Assignee::CurrentUser, show_all)?)?;
    let mut unassigned_issues: Vec<Issue> =
        serde_json::from_str(&run_gh_issue_list(repo, Assignee::None, show_all)?)?;

    assigned_issues.sort();
    unassigned_issues.sort();

    assigned_issues.append(&mut unassigned_issues);

    Ok(assigned_issues)
}

fn run_gh_issue_list(repo: &str, assignee: Assignee, show_all: bool) -> anyhow::Result<String> {
    let mut cmd = Command::new("gh");

    cmd.args([
        "issue",
        "list",
        "--label",
        "task",
        "--json",
        "number,title",
        "--state",
    ])
    .arg(if show_all { "all" } else { "open" })
    .arg("--repo")
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
