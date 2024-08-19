// (C) Psilocyborg 2024
// HENRII Car Diagnostic Tool
// File Description: Handlers for common commands, most commonly current
//                   data, readings from the ELM327 Adapter.


use crate::device::elm327::ElmDevice;




// Returns the current speed of the vehicle in kph
pub fn get_current_speed(mut device: ElmDevice) -> u8
{
    device.send_command("010D\r");

    let response : String = device.read().chars().filter(|&c| c != '>').collect();
    let parts : Vec<&str> = response.split_whitespace().collect();

    println!("{:?}", &parts);

    return u8::from_str_radix(parts[2], 16).expect("Whoops!");
} 


// Returns the current RPM of the vehicle.