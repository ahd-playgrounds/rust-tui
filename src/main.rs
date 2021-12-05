use std::{io, thread, time};
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::widgets::{Block, Borders};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let frame_rate = 3;
    let frame_duration: u64 = 1000 / frame_rate;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("spawned thread");
        loop {
            println!("waiting on input");
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer)  {
                Ok(_) => {
                    println!("char typed: {}", &buffer);
                    tx.send(buffer).unwrap();
                },
                Err(e) => {
                    panic!("could not read from stdin: {}", e);
                }
            }
        }
    });

    let mut counter = 0;

    loop {
        match rx.try_recv() {
            Ok(msg) => match msg.trim().as_ref() {
                "q" => panic!("ah"),
                _ => println!("received: '{}'", msg),
            }
            Err(e) => println!("nothing: {:?}", e),
        }

        // work out frame allowance
        let next_frame_time =
            time::Instant::now() + time::Duration::from_millis(frame_duration);

        counter += 1;
        println!("{}", counter);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref()
                )
                .split(f.size());

            let block = Block::default()
                .title(counter.to_string())
                .borders(Borders::ALL);

            f.render_widget(block, chunks[0]);


            let block = Block::default()
                .title("hello")
                .borders(Borders::ALL);

            f.render_widget(block, chunks[1]);
        })?;

        // TODO: look at now.elapsed
        if time::Instant::now() < next_frame_time {
            let remaining_frame_time = next_frame_time - time::Instant::now();

            thread::sleep(remaining_frame_time)
        }
    }

    Ok(())
}