extern crate device_query;

use device_query::{DeviceEvents, DeviceState};
use std::{
  fs::{read_to_string, write},
  thread::sleep,
  time::Duration,
};

const TIMEOUT: &str = "5000";
const TICK: u64 = 1000;
const RUN_FILE: &str = "/home/muesli/.kbblso";
const BRIGHTNESS_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness";
const BRIGHTNESS_SETTING_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness_hw_changed";

fn main() {
  write(RUN_FILE, TIMEOUT).unwrap_or(());
  let device_state = DeviceState::new();
  let _guard = device_state.on_key_down(|_| {
    write(RUN_FILE, TIMEOUT).unwrap_or(());
    write(
      BRIGHTNESS_PATH,
      read_to_string(BRIGHTNESS_SETTING_PATH).unwrap_or("0".to_string()),
    )
    .unwrap_or(());
  });

  loop {
    let timer = read_to_string(RUN_FILE).unwrap().parse::<u64>().unwrap();
    if timer == 0 {
      write(BRIGHTNESS_PATH, "0").unwrap_or(());
    } else {
      write(RUN_FILE, (timer - TICK).to_string()).unwrap_or(());
    }
    sleep(Duration::from_millis(TICK));
  }
}
