use rppal::uart::{Parity, Uart};
use std::path::Path;
use std::time;

fn main() {
    let usb_path = Path::new("/dev/ttyACM0");

    let mut uart = Uart::with_path(usb_path, 9_600, Parity::None, 8, 1).unwrap();

    loop {
        uart.set_read_mode(1, time::Duration::from_millis(500))
            .unwrap();

        let mut buffer = [0u8; 1];

        let msg = uart.read(&mut buffer).unwrap();

        println!("{:?}", msg);
    }
}
