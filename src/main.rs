extern crate device_query;

use device_query::{DeviceEvents, DeviceState};
use std::thread;
use std::time::Duration;

const RUN_FILE: &'static str = "/home/muesli/.kbblso";
const BRIGHTNESS_PATH: &'static str = "/sys/class/leds/asus::kbd_backlight/brightness";
const BRIGHTNESS_SETTING_PATH: &'static str =
  "/sys/class/leds/asus::kbd_backlight/brightness_hw_changed";

fn main() {
  std::fs::File::create(RUN_FILE).expect("Failed to create file :( ");
  std::fs::write(RUN_FILE, "5000").unwrap_or(());
  let device_state = DeviceState::new();
  let _guard = device_state.on_key_down(|_| {
    std::fs::write(RUN_FILE, "5000").unwrap_or(());
    std::fs::write(
      BRIGHTNESS_PATH,
      std::fs::read_to_string(BRIGHTNESS_SETTING_PATH).unwrap_or("0".to_string()),
    )
    .unwrap_or(());
  });

  loop {
    let timer = std::fs::read_to_string("./kbblso.tim")
      .unwrap()
      .parse::<i32>()
      .unwrap();
    if timer == 0 {
      std::fs::write(BRIGHTNESS_PATH, "0").unwrap_or(());
    } else {
      std::fs::write(RUN_FILE, (timer - 1000).to_string()).unwrap_or(());
    }
    thread::sleep(Duration::from_secs(1));
  }
}
