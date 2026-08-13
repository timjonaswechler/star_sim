use crate::protocol::{Ready, Response};
use std::{
    io::{self, BufRead, Write},
    sync::{
        Mutex,
        mpsc::{self, Receiver, SyncSender, TryRecvError},
    },
    thread,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Line(String),
    Eof,
    Error(String),
}

pub struct JsonLinesInput {
    receiver: Mutex<Receiver<Input>>,
}

impl JsonLinesInput {
    pub fn stdin(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::sync_channel(capacity);
        thread::spawn(move || read_lines(io::stdin().lock(), sender));
        Self {
            receiver: Mutex::new(receiver),
        }
    }

    pub fn from_receiver(receiver: Receiver<Input>) -> Self {
        Self {
            receiver: Mutex::new(receiver),
        }
    }

    pub fn try_recv(&self) -> Result<Input, TryRecvError> {
        self.receiver
            .lock()
            .expect("agent input mutex poisoned")
            .try_recv()
    }
}

fn read_lines(reader: impl BufRead, sender: SyncSender<Input>) {
    for line in reader.lines() {
        let input = match line {
            Ok(line) => Input::Line(line),
            Err(error) => Input::Error(error.to_string()),
        };
        let stop = matches!(input, Input::Error(_));
        if sender.send(input).is_err() || stop {
            return;
        }
    }
    let _ = sender.send(Input::Eof);
}

pub trait Output: Send + Sync + 'static {
    fn ready(&self, ready: &Ready) -> io::Result<()>;
    fn response(&self, response: &Response) -> io::Result<()>;
}

#[derive(Default)]
pub struct StdoutOutput;

impl Output for StdoutOutput {
    fn ready(&self, ready: &Ready) -> io::Result<()> {
        write_json(ready)
    }

    fn response(&self, response: &Response) -> io::Result<()> {
        write_json(response)
    }
}

fn write_json(value: &impl serde::Serialize) -> io::Result<()> {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    serde_json::to_writer(&mut lock, value).map_err(io::Error::other)?;
    writeln!(lock)?;
    lock.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn reader_reports_lines_then_eof() {
        let (sender, receiver) = mpsc::sync_channel(4);
        read_lines(Cursor::new("one\ntwo\n"), sender);
        assert_eq!(receiver.recv().unwrap(), Input::Line("one".into()));
        assert_eq!(receiver.recv().unwrap(), Input::Line("two".into()));
        assert_eq!(receiver.recv().unwrap(), Input::Eof);
    }
}
