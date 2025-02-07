#![no_std]
#![no_main]


use R9A02G021::port1::pcntr1::{Pdr07, Podr07};


extern crate panic_halt;

fn configuration(){
    let _led = Pdr07::_1; // para configurar como pin de salida
}

fn led_on(){
    let _led = Podr07::_1; //High output Encendido
    //self::Podr07::_1; 
    

}
fn led_off(){
    let _led = Podr07::_0;  //Low output Apagado

}

#[riscv_rt::entry]
fn main() -> ! {


    // configuración LED1 P107
    //let led = Pdr07W;
    configuration();
    led_on();
    
    loop {

        /*led_on();
        sleep.delay_ms(500); // 0.5s
        led_off();
        sleep.delay_ms(500); // 0.5s*/
    }

}
