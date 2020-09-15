use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::{BufRead, BufReader, ErrorKind};
use std::time;

fn main() {
    let button_reader_settings = serialport::SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: time::Duration::from_secs(1),
    };
    let button_reader =
        serialport::open_with_settings("/dev/ttyACM0", &button_reader_settings).unwrap();

    let clone = button_reader.try_clone().unwrap();
    let mut buf_reader = BufReader::new(button_reader);
    loop {
        let mut msg = String::new();

        match buf_reader.read_line(&mut msg) {
            Ok(_) => {
                println!("{:?}", msg);
                clone.clear(serialport::ClearBuffer::All).unwrap();
            }

            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),

            Err(e) => eprintln!("{:?}", e),
        };
    }
}
