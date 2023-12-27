use device_query::{DeviceEvents, DeviceState};
use std::{
  fs::{read_to_string, write},
  io::Result,
  sync::{Arc, Mutex},
};

const TIMEOUT: u64 = 6;
const BRIGHTNESS_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness";
const BRIGHTNESS_SETTING_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness_hw_changed";

fn main() -> Result<()> {
  let time_mtx = Arc::new(Mutex::new(std::time::SystemTime::now()));
  let time_cpy = Arc::clone(&time_mtx);
  let _callback_guard = DeviceState::new().on_key_down(move |_| {
    (|| -> Result<()> {
      let mut time = time_cpy.lock().unwrap();
      *time = std::time::SystemTime::now();
      write(BRIGHTNESS_PATH, read_to_string(BRIGHTNESS_SETTING_PATH)?)?;
      Ok(())
    })()
    .expect("I/O fail!")
  });
  loop {
    if std::time::SystemTime::elapsed(&Arc::clone(&time_mtx).lock().unwrap())
      .unwrap()
      .as_secs()
      > TIMEOUT
    {
      write(BRIGHTNESS_PATH, "0")?
    }
  }
}
