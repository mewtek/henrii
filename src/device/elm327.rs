// (C) Psilocyborg 2024
// HENRII Car Diagnostic Tool
// File Description: Functions for communication with an ELM327 Adapter over
//                   a serial connection.

// TODO : Add bluetooth & wireless dongle support, account for differing protocols within
//        certain vehicles' OBD-II standard type.


use serialport::{self, SerialPort};
use crate::logging::log::{Logger, LogLevel};

pub struct ElmDevice { 
    serial: Box<dyn SerialPort>,
    logger: Logger,
}

impl ElmDevice
{
    pub fn new(port: &str, baud_rate: u32, protocol: &i32) -> Self
    {
        let logger = Logger::new(LogLevel::Info);


        let serial = serialport::new(port, baud_rate)
                .flow_control(serialport::FlowControl::None)
                .data_bits(serialport::DataBits::Eight)
                .stop_bits(serialport::StopBits::One)
                .parity(serialport::Parity::None)
                .timeout(std::time::Duration::from_secs(2))
                .open().expect("Failed to open port");

        logger.log(LogLevel::Info, &format!("Connection to serial port established @ {:?}", serial.name()));

        ElmDevice
        {
            serial: serial,
            logger: logger
        }
    }
    
    // Reset the ELM327's CLI and set a few parameters to make it easier to read
    // the data coming from it 
    pub fn intialize(&mut self) {
        self.send_command("AT Z\r");

        std::thread::sleep(std::time::Duration::from_millis(200));

        self.send_command("AT H0\r");
        self.send_command("AT E0\r");
        self.send_command("AT L0\r");

        self.read();    // Clears the stdout for the ELM327
    }

    pub fn send_command(&mut self, command: &str) -> Vec<&str> {
        let output = command.as_bytes();
        self.serial.write(output).expect("Failed to write to ELM327 scanner");
        self.logger.log(LogLevel::Info, &format!("Sending command {}", command));
        std::thread::sleep(std::time::Duration::from_millis(100));
        
    }

    pub fn read(&mut self) -> String
    {
        let mut response: Vec<u8> = vec![0; 128];
        let bytes_read = self.serial.read(response.as_mut_slice()).expect("Failed to read from port.");

        self.serial.flush().expect("Unable to flush port");
        self.logger.log(LogLevel::Info, &format!("Bytes: {:?}", response));
        return String::from_utf8_lossy(&response[..bytes_read]).to_string();
        
    }
}