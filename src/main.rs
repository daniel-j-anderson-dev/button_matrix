use linux_embedded_hal::gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};

const CLEAR: &str = "\x1b[1;1H";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print!("{}", CLEAR);

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
        // find char
        let mut row_index = 0;
        let mut column_index = 0;
        'row: loop {
            rows[row_index].set_value(1)?;
        
            column_index = 0;
            'col: while column_index < 4 {
                let event = columns[column_index].get_event()?;
                println!("{:?}", event);
                match event.event_type() {
                    EventType::RisingEdge => {
                        println!("EventType::RisingEdge");
                        break 'row;
                    }
                    EventType::FallingEdge => println!("EventType::FallingEdge"),
                }
                column_index += 1;
            }


            rows[row_index].set_value(0)?;
            
            row_index += 1;
            if row_index >= 4 {
                row_index = 0;
            }

        }
        
        println!("({}, {})", row_index, column_index);

        // println!("({},{}) Button pressed", row_index, column_index);

        // process char
        // break;
    }

    return Ok(());
}
