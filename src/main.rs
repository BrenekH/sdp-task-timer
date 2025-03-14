use std::{collections::HashMap, env::args, fs, time::Duration};

use config::{load_config, Config};
use inquire::{Confirm, Select};
use serde::{Deserialize, Serialize};
use tui::App;

mod config;
mod github;
mod tui;

#[derive(Serialize, Deserialize)]
struct DataStore {
    tasks: HashMap<u64, Task>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    sessions: Vec<Session>,
}

#[derive(Serialize, Deserialize)]
struct Session {
    duration: Duration,
}

fn main() {
    let data_store_path = dirs::data_dir()
        .unwrap()
        .join("sdp-task-timer/data_store.json");
    fs::create_dir_all(data_store_path.parent().unwrap()).unwrap();

    let mut data_store: DataStore = serde_json::from_str(
        &fs::read_to_string(&data_store_path).unwrap_or("{\"tasks\": {}}".into()),
    )
    .unwrap();

    let show_all_issues = args().any(|arg| &arg == "--all");
    let cfg: Config = load_config().unwrap();

    let issue = Select::new(
        "Select an issue:",
        github::get_issue_list(&cfg.repository, show_all_issues).unwrap(),
    )
    .prompt()
    .unwrap();

    let default_task = Task {
        title: issue.title.clone(),
        sessions: vec![],
    };

    let time_spent =
        time_spent_on_task(data_store.tasks.get(&issue.number).unwrap_or(&default_task));

    println!(
        "You have spent {:.2} minutes on task #{}.\n",
        time_spent.as_secs_f64() / 60.0,
        issue.number
    );

    let start_new_session = Confirm::new("Would you like to start a new session?")
        .with_default(true)
        .prompt()
        .unwrap();
    if !start_new_session {
        return;
    }

    let mut terminal = ratatui::init();
    let mut app = App::new(&issue);
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result.unwrap();

    let task = data_store.tasks.entry(issue.number).or_insert(default_task);

    task.sessions.push(Session {
        duration: app.timer.total_duration,
    });

    fs::write(
        &data_store_path,
        serde_json::to_string_pretty(&data_store).unwrap(),
    )
    .unwrap();

    let time_spent = time_spent_on_task(data_store.tasks.get(&issue.number).unwrap());

    println!(
        "\nYou have now spent {:.2} minutes on task #{}.\n",
        time_spent.as_secs_f64() / 60.0,
        issue.number
    );
}

fn time_spent_on_task(task: &Task) -> Duration {
    task.sessions.iter().map(|s| s.duration).sum()
}
