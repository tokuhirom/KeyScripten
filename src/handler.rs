use std::collections::VecDeque;
use core_graphics::event::{CGEventFlags, CGKeyCode};
use std::sync::mpsc::SyncSender;
use std::ops::BitAnd;
use crate::event::Event;
use crate::key::Key;
use crate::keycodes::key_from_code;
use crate::KeyState;
use crate::sender::Sender;
use crate::state::State;

pub struct Handler {
    buffer: VecDeque<KeyState>,
    capacity: usize,
    playing: bool,
    latest_flags: CGEventFlags,
    shortcut_pressed: bool,
}

impl Handler {
    pub fn new(capacity: usize) -> Handler {
        Handler {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            playing: false,
            latest_flags: CGEventFlags::CGEventFlagNonCoalesced,
            shortcut_pressed: false,
        }
    }

    pub fn callback(&mut self, event: Event) -> Option<Event> {
        if self.playing {
            log::info!("Ignoring event due to playing macro now: {:?}", event);
            return Some(event);
        }

        match event {
            Event::KeyPress(code) => {
                if self.is_shortcut_pressed(code) {
                    log::info!("Shortcut key pressed!! 444");
                    self.shortcut_pressed = true;

                    let sender = Sender::new();
                    sender.process(State::new(
                        self.buffer.clone(),
                        self.latest_flags
                    ));
                    return None;
                }

                // fill buffer
                self.buffer.push_front(KeyState {
                    code,
                    flags: self.latest_flags
                });
                if self.capacity < self.buffer.len() {
                    self.buffer.pop_back();
                }
                log::info!("pressed~~~ {:?}", self.buffer);
            }
            Event::KeyRelease(_code) => {
            }
            Event::FlagsChanged(key, flags) => {
                log::info!("Flags changed: key={:?}, flags={:?}", key, flags);
                if self.shortcut_pressed && !self.is_modifier_pressing(flags) {
                    self.shortcut_pressed = false;
                    return Some(event);
                }

                self.latest_flags = flags;
            }
        }
        return Some(event);
    }

    fn is_modifier_pressing(&self, flags: CGEventFlags) -> bool {
        let modifiers = vec![
            CGEventFlags::CGEventFlagControl,
            CGEventFlags::CGEventFlagAlternate,
            CGEventFlags::CGEventFlagShift,
            CGEventFlags::CGEventFlagCommand,
        ];
        for modifier in modifiers {
            if !(flags & modifier).is_empty() {
                log::info!("Pressing moldifier: {:?}, {}", modifier, (flags & modifier).is_empty());
                return true;
            }
        }
        return false;
    }

    fn is_shortcut_pressed(&self, code: CGKeyCode) -> bool {
        // TODO: make this configurable
        log::info!("is_shortcut_pressed: {:?} code={:?}", self.latest_flags, code);
        if let flags = self.latest_flags {
            // TODO more smart bit comparison
            if flags.bitand(CGEventFlags::CGEventFlagControl).bits() > 0 &&
                flags.bitand(CGEventFlags::CGEventFlagAlternate).bits() == 0 &&
                flags.bitand(CGEventFlags::CGEventFlagShift).bits() == 0 &&
                flags.bitand(CGEventFlags::CGEventFlagCommand).bits() == 0 &&
                key_from_code(code) == Key::KeyJ {
                return true;
            }
        }
        return false;
    }
}
