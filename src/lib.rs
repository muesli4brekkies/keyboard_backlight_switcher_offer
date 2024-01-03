use std::fs::{read_to_string, write};
use std::time::{Duration, SystemTime};

use x11_keypress_detect::*;

enum Switch {
  On,
  Off,
}

impl Switch {
  fn br(self) {
    const PATH: &str = "/sys/class/leds/asus::kbd_backlight";
    let off: String = String::from("0");
    write(
      format!("{PATH}/brightness"),
      match self {
        Switch::Off => off,
        Switch::On => read_to_string(format!("{PATH}/brightness_hw_changed")).unwrap_or(off),
      },
    )
    .unwrap()
  }
}

struct LastPressTime(SystemTime);

impl LastPressTime {
  fn diff(&self) -> u64 {
    self.0.elapsed().unwrap().as_secs()
  }

  fn reset() -> Self {
    Self(SystemTime::now())
  }
}

pub fn detection_loop() {
  let mut last_press = LastPressTime::reset();
  let display = get_display();
  loop {
    if key_pressed(&display) {
      last_press = LastPressTime::reset();
      Switch::br(Switch::On);
    } else if last_press.diff() > 5 {
      Switch::br(Switch::Off);
    }
    std::thread::sleep(Duration::from_millis(500))
  }
}
