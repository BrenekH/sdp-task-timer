use std::{
    fmt::Display,
    io,
    process::Command,
    time::{Duration, Instant},
};

use inquire::Select;
use serde::{Deserialize, Serialize};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

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
                // AssigneeVariant::User(user) => "assignee".to_owned() + user,
            }
        ))
    }
}

#[derive(Debug)]
enum TimerStatus {
    Running,
    Stopped,
}

fn main() {
    let issue = Select::new("Select an issue to work on", get_issue_list().unwrap())
        .prompt()
        .unwrap();

    println!("Starting work on {issue}...\n");

    let mut terminal = ratatui::init();
    let app_result = App::new(issue).run(&mut terminal);
    ratatui::restore();
    app_result.unwrap();
}

#[derive(Debug)]
pub struct App {
    start_time: Instant,
    timer_status: TimerStatus,
    issue: Issue,
    exit: bool,
}

impl App {
    fn new(issue: Issue) -> Self {
        Self {
            start_time: Instant::now(),
            timer_status: TimerStatus::Running,
            issue,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        if !(event::poll(Duration::from_millis(250))?) {
            return Ok(());
        }

        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(
            format!(
                " Working on Task #{}: {} ",
                self.issue.number, self.issue.title
            )
            .bold(),
        );
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            get_timer_text(&self.start_time).yellow()
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn get_timer_text(start_time: &Instant) -> String {
    let elapsed_time = Instant::now() - *start_time;
    let elapsed_minutes = (elapsed_time.as_secs() / 60) as u64;
    let elapsed_seconds = elapsed_time.as_secs() - (elapsed_minutes * 60);

    format!("{:02}:{:02}", elapsed_minutes, elapsed_seconds)
}

fn get_issue_list() -> anyhow::Result<Vec<Issue>> {
    let mut assigned_issues: Vec<Issue> = serde_json::from_str(&run_gh_issue_list(
        "cs481-ekh/s25-sprout-squad",
        Assignee::CurrentUser,
    )?)?;
    let mut unassigned_issues: Vec<Issue> = serde_json::from_str(&run_gh_issue_list(
        "cs481-ekh/s25-sprout-squad",
        Assignee::None,
    )?)?;

    assigned_issues.sort();
    unassigned_issues.sort();

    assigned_issues.append(&mut unassigned_issues);

    Ok(assigned_issues)
}

fn run_gh_issue_list(repo: &str, assignee: Assignee) -> anyhow::Result<String> {
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
