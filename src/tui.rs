use std::{
    fmt::Display,
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use super::github::Issue;

#[derive(Debug, PartialEq, Eq)]
enum TimerStatus {
    Running,
    Stopped,
}

impl TimerStatus {
    fn action_text(&self) -> String {
        match self {
            TimerStatus::Running => " Pause ",
            TimerStatus::Stopped => " Resume ",
        }
        .into()
    }

    fn color_timer_text<'a>(&self, text: &'a str) -> Span<'a> {
        match self {
            TimerStatus::Running => text.green(),
            TimerStatus::Stopped => text.red(),
        }
    }
}

#[derive(Debug)]
pub struct Timer {
    pub total_duration: Duration,
    status: TimerStatus,
    start_time: Instant,
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dur = self.total_duration;

        if self.status == TimerStatus::Running {
            dur += Instant::now() - self.start_time;
        }

        let elapsed_minutes = dur.as_secs() / 60;
        let elapsed_seconds = dur.as_secs() - (elapsed_minutes * 60);

        f.write_fmt(format_args!(
            "{:02}:{:02}",
            elapsed_minutes, elapsed_seconds
        ))
    }
}

#[derive(Debug)]
pub struct App<'a> {
    pub timer: Timer,
    issue: &'a Issue,
    exit: bool,
}

impl<'a> App<'a> {
    pub fn new(issue: &'a Issue) -> Self {
        Self {
            timer: Timer {
                start_time: Instant::now(),
                status: TimerStatus::Running,
                total_duration: Duration::new(0, 0),
            },
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
            KeyCode::Char('p') => self.handle_pause(),
            _ => (),
        }
    }

    fn exit(&mut self) {
        self.exit = true;

        if self.timer.status == TimerStatus::Running {
            self.timer.total_duration += Instant::now() - self.timer.start_time;
        }
    }

    fn handle_pause(&mut self) {
        use TimerStatus::*;
        self.timer.status = match self.timer.status {
            Running => {
                self.timer.total_duration += Instant::now() - self.timer.start_time;
                Stopped
            }
            Stopped => {
                self.timer.start_time = Instant::now();
                Running
            }
        };
    }
}

impl<'a> Widget for &'a App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(
            format!(
                " Working on Task #{}: {} ",
                self.issue.number, self.issue.title
            )
            .bold(),
        );
        let instructions = Line::from(vec![
            self.timer.status.action_text().into(),
            "<P> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let t = self.timer.to_string();
        let timer_text = self.timer.status.color_timer_text(&t);
        let counter_text = Text::from(vec![Line::from(vec![timer_text])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
