use linux_embedded_hal::gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};

const CLEAR: &str = "\x1b[1;1H";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", CLEAR);

    let mut gpio_chip = Chip::new("/dev/gpiochip0")?;
    let rows = [
        gpio_chip.get_line(256)?.request(LineRequestFlags::OUTPUT, 0, "row0")?,
        gpio_chip.get_line(271)?.request(LineRequestFlags::OUTPUT, 0, "row1")?,
        gpio_chip.get_line(268)?.request(LineRequestFlags::OUTPUT, 0, "row2")?,
        gpio_chip.get_line(258)?.request(LineRequestFlags::OUTPUT, 0, "row3")?,
    ];
    let mut columns = [
        gpio_chip.get_line(272)?.events(LineRequestFlags::INPUT, EventRequestFlags::BOTH_EDGES, "column0")?,
        gpio_chip.get_line(257)?.events(LineRequestFlags::INPUT, EventRequestFlags::BOTH_EDGES, "column1")?,
        gpio_chip.get_line(260)?.events(LineRequestFlags::INPUT, EventRequestFlags::BOTH_EDGES, "column2")?,
        gpio_chip.get_line(259)?.events(LineRequestFlags::INPUT, EventRequestFlags::BOTH_EDGES, "column3")?,
    ];

    'keypad: loop {
        print!("{}", CLEAR);

        // find char
        'row: for row in rows.iter().cycle() {
            print!("{}", CLEAR);

            row.set_value(1)?;
        
            for column in columns.iter_mut() {
                print!("{}", CLEAR);
    
                let event = column.get_event()?;
    
                dbg!(&event);
    
                match event.event_type() {
                    EventType::RisingEdge => break 'row,
                    EventType::FallingEdge => {},
                }
            }
    
            row.set_value(0)?;
        }
        
        // process char
        // break;
    }

    return Ok(());
}
