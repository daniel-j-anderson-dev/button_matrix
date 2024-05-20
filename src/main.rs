use linux_embedded_hal::gpio_cdev::Chip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chip = Chip::new("/dev/gpiochip0")?;

    return Ok(());
}
