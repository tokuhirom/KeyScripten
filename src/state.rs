use std::collections::VecDeque;
use apple_sys::CoreGraphics::CGEventFlags;
use crate::KeyState;

#[derive(Debug)]
pub struct State {
    pub(crate) buffer: VecDeque<KeyState>,
    pub(crate) flags: CGEventFlags,
}

impl State {
    pub fn new(buffer: VecDeque<KeyState>, flags: CGEventFlags) -> Self {
        State {
            buffer,
            flags
        }
    }
}
