use nanohat_oled::{Oled, OledResult};

fn main() -> OledResult {
    println!("--- enter main rs ---");
    let mut oled = Oled::from_path("/dev/i2c-2")?;
    oled.init()?;
    oled.put_string("Hello, world!")?;
    println!("--- leave main rs ---");
    Ok(())
}