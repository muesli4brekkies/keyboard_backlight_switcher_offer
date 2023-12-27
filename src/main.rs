extern crate device_query;

use device_query::{DeviceEvents, DeviceState, Keycode};
use std::{
  fs::{read_to_string, write},
  io::Result,
  thread::sleep,
  time::Duration,
};

const TIMEOUT: &str = "6";
const TICK: u64 = 3;
const RUN_FILE: &str = "/home/muesli/.kbblso";
const BRIGHTNESS_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness";
const BRIGHTNESS_SETTING_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness_hw_changed";

fn callback(_: &Keycode) {
  (|| -> Result<()> {
    write(RUN_FILE, TIMEOUT)?;
    write(BRIGHTNESS_PATH, read_to_string(BRIGHTNESS_SETTING_PATH)?)?;
    Ok(())
  })()
  .expect("Failed to write to file")
}

fn switch() -> Result<()> {
  let timer = std::fs::read_to_string(RUN_FILE)?
    .parse::<u64>()
    .unwrap_or(0);
  match timer {
    0 => write(BRIGHTNESS_PATH, "0")?,
    _ => write(RUN_FILE, (timer - TICK).to_string())?,
  }
  sleep(Duration::from_secs(TICK));
  Ok(())
}

fn main() -> Result<()> {
  write(RUN_FILE, TIMEOUT)?;
  let _callback_guard = DeviceState::new().on_key_down(callback);
  loop {
    switch()?
  }
}
