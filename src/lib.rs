pub fn scroll() -> !{
    let mut pos = 0;
    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    loop {
        if pos == -260 {
            pos = 0;
        }
        display.clear_buffer();
        Text::new(
            "hi caroline, i love you <3",
            Point::new(pos, 20),
            style,
        )
        .draw(&mut display)
        .unwrap();
        pos -= 1;
        display.flush().unwrap();

        delay.start(5.millis());
        let _ = nb::block!(delay.wait());
    }
}
