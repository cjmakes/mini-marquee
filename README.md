# mini-marquee
I wanted to try some embedded rust and found some well supported components. Here is what I came up with.

![demo](demo.webp)

## Things I like about this project
- it works!
- rust compiler caught a bug I had where I wanted to use an RGB font with a monochrome display
- the simulator is great
- being able to share the same library between physical and simulation is great
- well supported board means it was plug-and-play
- actual board is great to work with, usb-c, integrated led display
- embedded-hal traits are incredible. My board supplies an interface that
  implements i2c, the display driver takes something that implements i2c. The
  maintainer of the board crate and the maintainer of the display driver crate
  never had to coordinate.
- embedded-graphics is amazing, so much being able to draw fonts and
  shapes without needing to learn a custom library

## Things I don't like
- ...nothing? 

## Using
### Simulation
```
cargo run -p simulation
```

### Flashing
Connect the board with:
```
sck - gp15
sda - gp14
```

Reset the board by holding reset, holding boot, releasing reset, releasing boot.

Mount the board
```
mount /dev/sda1 /mnt
```

Flash, note, we need to cd to physical to get cargo to pickup the correct compile target
```
cd physical; cargo run --release
```
