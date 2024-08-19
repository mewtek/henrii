mod logging;
mod device;
mod obd;

use device::elm327;

fn main()
{
    let mut adapter = elm327::ElmDevice::new("COM4", 115200, &1);
    adapter.intialize();

    println!("{}", obd::commands::get_current_speed(adapter));
}