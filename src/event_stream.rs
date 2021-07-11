use std::io::BufRead;

use crate::errors::Error;
use crate::errors::Result;
use crate::entities::event::Event;
use crate::entities::notification::Notification;
use crate::entities::status::Status;

use tungstenite::client::AutoStream;

#[derive(Debug)]
/// WebSocket newtype so that EventStream can be implemented without coherency
/// issues
pub struct WebSocket(pub(crate) tungstenite::protocol::WebSocket<AutoStream>);

/// A type that streaming events can be read from
pub trait EventStream {
    /// Read a message from this stream
    fn read_message(&mut self) -> Result<String>;
}

impl<R: BufRead> EventStream for R {
    fn read_message(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.read_line(&mut buf)?;
        Ok(buf)
    }
}

impl EventStream for WebSocket {
    fn read_message(&mut self) -> Result<String> {
        self.0.read_message()?.into_text().map_err(Error::from)
    }
}

#[derive(Debug)]
/// Iterator that produces events from a mastodon streaming API event stream
pub struct EventReader<R: EventStream>(pub(crate) R);
impl<R: EventStream> Iterator for EventReader<R> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::new();
        loop {
            if let Ok(line) = self.0.read_message() {
                let line = line.trim().to_string();
                if line.starts_with(':') || line.is_empty() {
                    continue;
                }
                lines.push(line);
                if let Ok(event) = self.make_event(&lines) {
                    lines.clear();
                    return Some(event);
                } else {
                    continue;
                }
            }
        }
    }
}

impl<R: EventStream> EventReader<R> {
    fn make_event(&self, lines: &[String]) -> Result<Event> {
        let event;
        let data;
        if let Some(event_line) = lines.iter().find(|line| line.starts_with("event:")) {
            event = event_line[6..].trim().to_string();
            data = lines
                .iter()
                .find(|line| line.starts_with("data:"))
                .map(|x| x[5..].trim().to_string());
        } else {
            use serde::Deserialize;
            #[derive(Deserialize)]
            struct Message {
                pub event: String,
                pub payload: Option<String>,
            }
            let message = serde_json::from_str::<Message>(&lines[0])?;
            event = message.event;
            data = message.payload;
        }
        let event: &str = &event;
        Ok(match event {
            "notification" => {
                let data = data.ok_or_else(|| {
                    Error::Other("Missing `data` line for notification".to_string())
                })?;
                let notification = serde_json::from_str::<Notification>(&data)?;
                Event::Notification(notification)
            }
            "update" => {
                let data =
                    data.ok_or_else(|| Error::Other("Missing `data` line for update".to_string()))?;
                let status = serde_json::from_str::<Status>(&data)?;
                Event::Update(status)
            }
            "delete" => {
                let data =
                    data.ok_or_else(|| Error::Other("Missing `data` line for delete".to_string()))?;
                Event::Delete(data)
            }
            "filters_changed" => Event::FiltersChanged,
            _ => return Err(Error::Other(format!("Unknown event `{}`", event))),
        })
    }
}

