use std::{
  fs::{read_to_string, write},
  ptr, slice,
  thread::sleep,
  time::{Duration, SystemTime},
};
use x11::xlib::{self, _XDisplay};

const PATH: &str = "/sys/class/leds/asus::kbd_backlight";
fn write_brightness(turn_off: bool) {
  write(
    format!("{PATH}/brightness"),
    match turn_off {
      true => String::from("0"),
      false => read_to_string(format!("{PATH}/brightness_hw_changed")).unwrap_or(String::from("0")),
    },
  )
  .unwrap()
}

fn key_pressed(display: *mut _XDisplay) -> bool {
  let keymap = [0; 32].as_mut_ptr();
  unsafe {
    xlib::XQueryKeymap(display, keymap);
    slice::from_raw_parts(keymap, 32)
  }
  .iter()
  .any(|byte| *byte != 0)
}

fn get_display() -> *mut _XDisplay {
  match unsafe { xlib::XOpenDisplay(ptr::null()) } {
    display if unsafe { display.as_ref().is_some() } => display,
    _ => panic!("Could not connect to a X display"),
  }
}
fn main() {
  let display = get_display();
  let mut last_keypress_time = SystemTime::now();
  loop {
    if key_pressed(display) {
      last_keypress_time = SystemTime::now();
      write_brightness(false)
    }
    if last_keypress_time.elapsed().unwrap() > Duration::from_secs(5) {
      write_brightness(true)
    }
    sleep(Duration::from_millis(500));
  }
}
