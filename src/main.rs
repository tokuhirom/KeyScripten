mod send;
mod keycodes;
mod grab;
mod common;
mod event;
mod key;

use std::collections::VecDeque;
use std::ops::BitAnd;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;
use std::time::Duration;
use core_graphics::event::{CGEventFlags, CGKeyCode};
use simplelog::ColorChoice;
use crate::common::CGEventTapCreate;
use crate::event::Event;
use crate::grab::grab_ex;
use crate::key::Key;
use crate::keycodes::key_from_code;
use crate::send::simulate_ex;

fn main() -> anyhow::Result<()> {
    let config = simplelog::ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("Cannot get timezone")
        .build();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto
        ),
    ])?;

    let (tx, rx) = sync_channel(7);

    thread::spawn(move || {
        let sender = Sender::new();

        loop {
            match rx.recv() {
                Ok(buffer) => {
                    log::info!("buffer={:?}", buffer);
                    sender.process(buffer);
                }
                Err(err) => {
                    log::error!("Cannot receive event: {:?}", err);
                }
            }
        }
    });

    let mut handler = Handler::new(64, tx);
    if let Err(error) = grab_ex(move |event| {
        handler.callback(event)
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}

struct Handler {
    buffer: VecDeque<CGKeyCode>,
    capacity: usize,
    playing: bool,
    latest_flags: Option<CGEventFlags>,
    shortcut_pressed: bool,
    tx: SyncSender<VecDeque<CGKeyCode>>,
}

impl Handler {
    fn new(capacity: usize, tx: SyncSender<VecDeque<CGKeyCode>>) -> Handler {
        Handler {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            playing: false,
            latest_flags: None,
            shortcut_pressed: false,
            tx,
        }
    }

    fn callback(&mut self, event: Event) -> Option<Event> {
        if self.playing {
            log::info!("Ignoring event due to playing macro now: {:?}", event);
            return Some(event);
        }

        match event {
            Event::KeyPress(code) => {
                if self.is_shortcut_pressed(code) {
                    log::info!("Shortcut key pressed!! 444");
                    self.shortcut_pressed = true;
                    if let Err(err) = self.tx.send(self.buffer.clone()) {
                        log::error!("Cannot send message to the execution thread: {}", err);
                    }
                    return None;
                }

                // fill buffer
                self.buffer.push_front(code);
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
                    // if let Err(err) = self.tx.send(self.buffer.clone()) {
                    //     log::error!("Cannot send message to the execution thread: {}", err);
                    // }
                    return Some(event);
                }

                self.latest_flags = Some(flags);
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
        if let Some( flags) = self.latest_flags {
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

struct Sender {
}

impl Sender {
    fn new() -> Self {
        Sender {
        }
    }

    pub fn process(&self, buffer: VecDeque<CGKeyCode>) {
        if let Some(size) = self.check_repeat(&buffer) {
            log::info!("Repeat count: {}", size);

            self.send_event(&Event::FlagsChanged(0, CGEventFlags::CGEventFlagNonCoalesced));

            let front = &buffer.as_slices().0[0..size];
            for code in front.iter().rev() {
                self.send_event(&Event::KeyPress(*code));
            }
        } else {
            log::warn!("No repeats!!!");
        }
    }

    fn check_repeat(&self, buffer: &VecDeque<CGKeyCode>) -> Option<usize> {
        for size in (1..=buffer.len() / 2).rev() {
            let front = &buffer.as_slices().0[0..size];
            let rear = &buffer.as_slices().0[size..size*2];
            log::info!("front={:?} rear={:?}", front, rear);
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
