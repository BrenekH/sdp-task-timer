use std::{io::Write, thread, time::{Duration, Instant}};

fn main() {
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
