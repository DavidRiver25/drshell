use super::super::commands::{self, Api};
use super::Rustyline;

use rustyline::{
    Cmd, ConditionalEventHandler, Event, EventContext, KeyCode, KeyEvent, RepeatCount,
};

impl ConditionalEventHandler for Rustyline {
    fn handle(
        &self,
        evt: &Event,
        _n: RepeatCount,
        _positive: bool,
        _ctx: &EventContext,
    ) -> Option<Cmd> {
        if let Event::KeySeq(seq) = evt {
            if seq.len() == 1 {
                match seq[0] {
                    KeyEvent(KeyCode::Up, _) => {
                        let _ = commands::api(Api::DisplayPreviousCmd);
                        return Some(Cmd::Noop);
                    }
                    KeyEvent(KeyCode::Down, _) => {
                        let _ = commands::api(Api::DisplayNextCmd);
                        return Some(Cmd::Noop);
                    }
                    _ => {}
                }
            }
        }
        None
    }
}
