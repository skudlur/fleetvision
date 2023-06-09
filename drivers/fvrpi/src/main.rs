use rppal::gpio::{Gpio, InputPin, Level};
use rppal::i2c::I2c;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::thread;
use std::time::Duration;

pub mod utils;

const IR_SENSOR_PIN: u8 = 17;
const I2C_DEV_ADDRESS: u16 = 0x5A;
const SENSOR_PARAMETERS_NO: usize = 2;

fn read_prox_sensor(pin: &InputPin) -> f32 {
    // Function to read IR sensor
    let mut res = 0.0;
    loop {
        if pin.read() == Level::High {
            res = 0.0;
        } else {
            res = 1.0;
        }
        thread::sleep(Duration::from_millis(100));
        return res;
    }
}

fn display_sensor_data() {
    // Function to read temperature sensor data at real-time
    let gpio = Gpio::new().unwrap();
    let mut i2c = I2c::new().unwrap();

    i2c.set_slave_address(I2C_DEV_ADDRESS).unwrap();
    let _ir_pin = gpio.get(IR_SENSOR_PIN).unwrap().into_input_pullup();

    let mut inc = 1;
    let mut sensor_data_arr: [f32; SENSOR_PARAMETERS_NO] = [0.0; 2];
    let mut sensor_parameters_arr: [f32; SENSOR_PARAMETERS_NO] = [0.0; 2];

    utils::print_type_of(&sensor_data_arr);

    loop {
        let ambient_temp = read_temperature(&mut i2c);
        //let object_temp = read_temperature(&mut i2c, OBJ_TEMP_REG);
        let ir_distance = read_prox_sensor(&_ir_pin);

        sensor_parameters_arr = [ambient_temp, ir_distance];

        println!("{:?}", inc);
        inc += 1;
        println!("Ambient temperature: {:.2}C", ambient_temp);
        //println!("Object temperature: {:.2}C", object_temp);
        println!("IR Intrusion: {:.2}", ir_distance);

        for x in 0..SENSOR_PARAMETERS_NO {
            sensor_data_arr[x] = sensor_parameters_arr[x];
        }

        //utils::log_sensor_data(sensor_data_arr);

        thread::sleep(Duration::from_millis(500));
    }
}

fn read_temperature(i2c: &mut I2c) -> f32 {
    // Function to read and parse temperature
    i2c.write(&[0x07]).unwrap();
    thread::sleep(Duration::from_millis(500));

    let mut buf = [0u8; 2];
    i2c.read(&mut buf).unwrap();

    let temp_raw = (buf[1] as u16) << 8 | buf[0] as u16;
    let temp_c = ((temp_raw as f32) * 0.02 - 273.15) as f32;
    temp_c
}

fn main() {
    // Main function to run stuff

    display_sensor_data();
}
