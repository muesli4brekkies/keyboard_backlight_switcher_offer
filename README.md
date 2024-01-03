# Keyboard Backlight Switcher Offer

## Switches off the keyboard backlight after a little bit

* probably only works on some asus laptops.
* definitely only works on Linux.
* you could probably edit the file path for your hardware (/sys/class/leds/tpacpi::kbd_backlight/ for thinkpads, for instance).
* that file needs permissions. I recommend changing the ownership to root:video and adding you user to the video group. Put it in your .bashrc or something.
* requires x11.
* has a 0.000001% chance to delete everything on your PC.
