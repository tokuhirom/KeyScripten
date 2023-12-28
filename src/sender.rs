use core_graphics::event::CGEventFlags;
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;
use crate::event::Event;
use crate::KeyState;
use crate::send::simulate_ex;
use crate::state::State;

pub struct Sender {
}

impl Sender {
    pub fn new() -> Self {
        Sender {
        }
    }

    pub fn process(&self, state: State) {
        let buffer = state.buffer;
        if let Some(size) = self.check_repeat(&buffer) {
            log::info!("Repeat count: {}", size);

            // clear flags state
            self.send_event(&Event::FlagsChanged(0, CGEventFlags::CGEventFlagNonCoalesced));

            let front = &buffer.as_slices().0[0..size];
            for key_state in front.iter().rev() {
                self.send_event(&Event::KeyPress(key_state.code));
            }

            // restore
            self.send_event(&Event::FlagsChanged(0, state.flags));
        } else {
            log::warn!("No repeats!!!");
        }
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

    fn send_event(&self, event_type: &Event) {
        log::info!("Sending event: {:?}", event_type);

        match simulate_ex(event_type) {
            Ok(()) => (),
            Err(err) => {
                log::error!("We could not send {:?}: {:?}", event_type, err);
            }
        }

        // Let ths OS catchup (at least MacOS)
        let delay = Duration::from_millis(50);
        thread::sleep(delay);
    }
}
