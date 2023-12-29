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

fn check_keys(display: *mut _XDisplay) -> bool {
  let keymap = [0; 32].as_mut_ptr();
  unsafe {
    xlib::XQueryKeymap(display, keymap);
    slice::from_raw_parts(keymap, 32)
  }
  .iter()
  .fold(false, |a, b| if *b == 0 { a } else { true })
}

fn get_display() -> *mut _XDisplay {
  let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
  match unsafe { display.as_ref().is_some() } {
    true => display,
    false => panic!("Could not connect to a X display"),
  }
}
fn main() {
  let display = get_display();
  let mut last_keypress_time = SystemTime::now();
  loop {
    dbg!(last_keypress_time);
    if check_keys(display) {
      last_keypress_time = SystemTime::now();
      write_brightness(false)
    }
    if last_keypress_time.elapsed().unwrap() > Duration::from_secs(5) {
      write_brightness(true)
    }
    sleep(Duration::from_millis(500));
  }
}
