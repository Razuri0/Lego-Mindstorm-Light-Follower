/* 
Light Follower Robot Rust example
Author: David Laurent Reinhardt
Course: "System Design Projekt", Winter Semester 2025/2026

Description:
This program implements a simple light-following robot using a proportional controller.
The robot uses two light sensors to detect the intensity of light on its left and right sides.
Based on the difference in light intensity, it adjusts its motor speed to turn towards the light source.
*/

extern crate ev3dev_lang_rust;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{LightSensor, Sensor, SensorPort};
use ev3dev_lang_rust::Ev3Result;
use std::thread;
use std::time::Duration;


//  p(roportional)-controller
fn p_controller(sensor_left: i32, sensor_right: i32, p_term: f32) -> Ev3Result<i32> {
    let difference: i32 = sensor_left - sensor_right;
    Ok((p_term * difference as f32) as i32)
}


fn clamp(v: i32, min: i32, max: i32) -> i32 {
    if v < min { min } else if v > max { max } else { v }
}


//  "task main()" is executed at programm start
fn main() -> Ev3Result<()> {

    // Constants
    let light_left = LightSensor::get(SensorPort::In1)?;
    let light_right = LightSensor::get(SensorPort::In2)?;
    let motor = LargeMotor::get(MotorPort::OutA)?;
    let p_term: f32 = 0.1; // for some reason when using rust the sensors goes from 0 to 1000
                           //  while in Python they go from 0 to 100
                           //  so we need to adjust the p_term accordingly

    // variable to store p_controller output
    motor.run_direct()?;
    light_left.set_mode_reflect()?;
    light_right.set_mode_reflect()?;
    let mut motor_power: i32;

    //  loops forever
    loop {
        // read sensors, compute p_controller output and set motor speed
        motor_power = p_controller(light_left.get_value(0)?, light_right.get_value(0)?, p_term)?;
        motor.set_duty_cycle_sp(clamp(motor_power, -100, 100))?;
        thread::sleep(Duration::from_millis(100));
    }
}