use std::{
  fs::{read_to_string, write},
  os::raw::c_char,
  ptr, slice,
  thread::sleep,
  time::{Duration, SystemTime},
};
use x11::xlib::{self, _XDisplay};

fn write_brightness(turn_off: bool) {
  let path = "/sys/class/leds/asus::kbd_backlight";
  write(
    format!("{path}/brightness"),
    match turn_off {
      true => String::from("0"),
      false => read_to_string(format!("{path}/brightness_hw_changed")).unwrap(),
    },
  )
  .unwrap()
}

fn check_keys(display: *mut _XDisplay) -> bool {
  unsafe {
    if display.as_ref().is_none() {
      panic!("Could not connect to a X display");
    }
    let keymap: *mut c_char = [0; 32].as_mut_ptr();
    xlib::XQueryKeymap(display, keymap);
    slice::from_raw_parts(keymap, 32)
      .iter()
      .fold(false, |a, b| if *b == 0 { a } else { true })
  }
}

fn main() {
  unsafe {
    let display = xlib::XOpenDisplay(ptr::null());
    let mut last_keypress_time = SystemTime::now();
    loop {
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
}
