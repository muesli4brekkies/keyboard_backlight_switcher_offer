use std::fs::{read_to_string, write};
use std::time::SystemTime;

use x11_keypress_detect::KeypressDetect;

const PATH: &str = "/sys/class/leds/asus::kbd_backlight";
fn write_br(turn_off: bool) {
  let off: String = String::from("0");
  write(
    format!("{PATH}/brightness"),
    match turn_off {
      true => off,
      false => read_to_string(format!("{PATH}/brightness_hw_changed")).unwrap_or(off),
    },
  )
  .unwrap()
}

fn reset() -> SystemTime {
  SystemTime::now()
}

pub fn detection_loop() {
  let mut t_last_press = reset();
  let display = KeypressDetect::get_display();
  loop {
    if KeypressDetect::key_pressed(&display) {
      t_last_press = reset();
      write_br(false);
    } else if t_last_press.elapsed().unwrap().as_secs() > 5 {
      write_br(true);
    }
  }
}
