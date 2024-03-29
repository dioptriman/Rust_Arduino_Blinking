#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[arduino_hal::entry]
fn potato() -> !{
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();
    let pir = pins.d12.into_floating_input();
    
    loop{
        if pir.is_high(){
             led.set_high();
             arduino_hal::delay_ms(2000);
        } else if pir.is_low(){
            led.set_low();
            arduino_hal::delay_ms(2000);
        }
    }
}