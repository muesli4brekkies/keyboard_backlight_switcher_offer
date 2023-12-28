use device_query::{DeviceEvents, DeviceState, Keycode};
use std::{
  fs::{read_to_string, write},
  sync::{Arc, Mutex},
  thread::sleep,
  time::{Duration, SystemTime},
};

fn write_brightness(turn_off: bool) {
  let path = "/sys/class/leds/asus::kbd_backlight/";
  write(
    format!("{path}/brightness"),
    match turn_off {
      true => String::from("0"),
      false => read_to_string(format!("{path}/brightness_hw_changed")).unwrap(),
    },
  )
  .unwrap()
}

fn main() {
  let last_keypress_time = Arc::new(Mutex::new(SystemTime::now()));
  let guard_copy = last_keypress_time.clone();
  let callback = move |_: &Keycode| {
    *guard_copy.lock().unwrap() = SystemTime::now();
    write_brightness(false);
  };
  let _callback_guard = DeviceState::new().on_key_down(callback);
  loop {
    if last_keypress_time.lock().unwrap().elapsed().unwrap() > Duration::from_secs(6) {
      write_brightness(true)
    }
    sleep(Duration::from_secs(3));
  }
}
