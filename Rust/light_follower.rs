/* 
Light Follower Robot NXC example
Author: David Laurent Reinhardt
Course: "System Design Projekt", Winter Semester 2025/2026

Description:
This program implements a simple light-following robot using a proportional controller.
The robot uses two light sensors to detect the intensity of light on its left and right sides.
Based on the difference in light intensity, it adjusts its motor speed to turn towards the light source.
*/

extern crate ev3dev_lang_rust;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{LightSensor, SensorPort};
use ev3dev_lang_rust::Ev3Result;


//  p(roportional)-controller
fn p_controller(sensor_left: &LightSensor, sensor_right: &LightSensor, p_term: f32) -> Ev3Result<i32> {
    let difference: i32 = sensor_left.value(0)? - sensor_right.value(0)? as i32;
    Ok((p_term * difference as f32) as i32)
}


//  "task main()" is executed when the program starts
fn main() -> Ev3Result<()> {

    // Constants
    let LIGHT_LEFT = LightSensor::new(SensorPort::In1)?;
    let LIGHT_RIGHT = LightSensor::new(SensorPort::In2)?;
    let MOTOR = LargeMotor::new(MotorPort::OutA)?;
    let P_TERM: f32 = 1.0;

    // variable to store p_controller output
    let mut motor_power: i32;

    //  loops forever
    loop {
        motor_power = p_controller(&LIGHT_LEFT, &LIGHT_RIGHT, P_TERM)?;
        MOTOR.set_speed(motor_power)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}