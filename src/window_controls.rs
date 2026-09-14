use std::sync::atomic::{AtomicBool, Ordering};

pub struct WindowControls;

impl WindowControls {
    pub fn is_fullscreen() -> bool {
        FULLSCREEN.load(Ordering::Relaxed)
    }

    pub fn set_fullscreen(value: bool) {
        FULLSCREEN.store(value, Ordering::Relaxed);
        macroquad::window::set_fullscreen(value);
    } 
}

static FULLSCREEN: AtomicBool = AtomicBool::new(true);
