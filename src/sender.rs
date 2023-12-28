use core_graphics::event::CGEventFlags;
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;
use crate::event::Event;
use crate::KeyState;
use crate::send::{send_flags_changed_event, send_keyboard_event};
use crate::state::State;

pub struct Sender {
}

impl Sender {
    pub fn new() -> Self {
        Sender {
        }
    }

    pub fn process(&self, state: State) -> anyhow::Result<()> {
        let buffer = state.buffer;
        if let Some(size) = self.check_repeat(&buffer) {
            log::warn!("Repeat count: {}", size);

            // clear flags state
            send_flags_changed_event(CGEventFlags::CGEventFlagNonCoalesced)?;

            let front = &buffer.as_slices().0[0..size];
            for key_state in front.iter().rev() {
                send_keyboard_event(key_state.code, true)?;
            }

            // restore
            send_flags_changed_event(state.flags)?;
        } else {
            log::warn!("No repeats!!!: {:?}", buffer);
        }
        Ok(())
    }

    fn check_repeat(&self, buffer: &VecDeque<KeyState>) -> Option<usize> {
        for size in (1..=buffer.len() / 2).rev() {
            let front = &buffer.as_slices().0[0..size];
            let rear = &buffer.as_slices().0[size..size*2];
            // log::info!("front={:?} rear={:?}", front, rear);
            if front == rear {
                return Some(size);
            }
        }
        None
    }
}
