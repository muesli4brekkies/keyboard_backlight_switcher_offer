use device_query::{DeviceEvents, DeviceState, Keycode};
use std::{
  env,
  fs::{read_to_string, write},
  io::Result,
  sync::{Arc, Mutex},
  thread::sleep,
  time::{Duration, SystemTime},
};

fn write_brightness(turn_off: bool) -> Result<()> {
  let path = match env::args().any(|arg| arg.starts_with('-') && arg.contains("thinkpad")) {
    true => "/sys/class/leds/tcpacpi::kbd_backlight/",
    false => "/sys/class/leds/asus::kbd_backlight/",
  };
  write(
    format!("{path}/brightness"),
    match turn_off {
      true => String::from("0"),
      false => read_to_string(format!("{path}/brightness_hw_changed"))?,
    },
  )
}

fn main() -> Result<()> {
  let mtx = Arc::new(Mutex::new(SystemTime::now()));
  let guard_mtx = mtx.clone();
  let callback = move |_: &Keycode| {
    (|| -> Result<()> {
      let mut time = guard_mtx.lock().unwrap();
      *time = SystemTime::now();
      write_brightness(false)?;
      Ok(())
    })()
    .expect("I/O fail!")
  };
  let _callback_guard = DeviceState::new().on_key_down(callback);
  loop {
    if SystemTime::elapsed(&mtx.lock().unwrap()).unwrap() > Duration::from_secs(6) {
      write_brightness(true)?
    }
    sleep(Duration::from_secs(3));
  }
}
