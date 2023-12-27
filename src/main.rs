use device_query::{DeviceEvents, DeviceState};
use std::{
  env,
  fs::{read_to_string, write},
  io::Result,
  sync::{Arc, Mutex},
  thread::sleep,
  time::{Duration, SystemTime},
};

const TIMEOUT: u64 = 6;
const TICK: u64 = 3;
const BRIGHTNESS_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness";
const BRIGHTNESS_SETTING_PATH: &str = "/sys/class/leds/asus::kbd_backlight/brightness_hw_changed";

fn get_brightness_path() -> String {
  env::args()
    .enumerate()
    .fold(None, |a, (i, arg)| match arg == "-p" {
      true => match env::args().nth(i + 1) {
        Some(r) => Some(r),
        None => a,
      },
      false => a,
    })
    .unwrap_or(String::from(BRIGHTNESS_PATH))
}

fn main() -> Result<()> {
  let write_brightness =
    |contents: String| -> Result<()> { write(get_brightness_path(), contents) };
  let time_mtx = Arc::new(Mutex::new(SystemTime::now()));
  let time_cpy = Arc::clone(&time_mtx);
  let _callback_guard = DeviceState::new().on_key_down(move |_| {
    (|| -> Result<()> {
      let mut time = time_cpy.lock().unwrap();
      *time = SystemTime::now();
      write_brightness(read_to_string(BRIGHTNESS_SETTING_PATH)?)?;
      Ok(())
    })()
    .expect("I/O fail!")
  });
  loop {
    if SystemTime::elapsed(&Arc::clone(&time_mtx).lock().unwrap())
      .unwrap()
      .as_secs()
      > TIMEOUT
    {
      write_brightness(String::from("0"))?
    }
    sleep(Duration::from_secs(TICK));
  }
}
