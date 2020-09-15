use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::{BufRead, BufReader};
use std::time;

fn main() {
    let button_reader_settings = serialport::SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: time::Duration::from_millis(1),
    };
    let button_reader =
        serialport::open_with_settings("/dev/ttyACM0", &button_reader_settings).unwrap();
    let mut buf_reader = BufReader::new(button_reader);
    loop {
        let mut msg = String::new();

        buf_reader.read_line(&mut msg).unwrap();

        println!("{:?}", msg);
    }
}
