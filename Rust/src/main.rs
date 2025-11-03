/*
Light Follower Robot Rust example
Author: David Laurent Reinhardt
Course: "System Design Projekt", Winter Semester 2025/2026

Description:
This program implements a simple light-following robot using a pid-controller.
The robot uses two light sensors to detect the intensity of light on its left and right sides.
Based on the difference in light intensity, it adjusts its motor speed to turn towards the light source.
*/

extern crate ev3dev_lang_rust;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{LightSensor, SensorPort};
use ev3dev_lang_rust::Ev3Result;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};


//  PID Controller function
fn pid_controller(sensor_left: i32, sensor_right: i32, dt: f32, integral: &mut f32, prev_error: &mut f32, 
                kp: &f32, ki: &f32, kd: &f32) -> Ev3Result<i32> {
    let error: f32 = (sensor_left - sensor_right) as f32;

    *integral += error * dt;

    let mut derivative: f32 = 0.0;
    if dt > 0.0 {
        derivative = (error - *prev_error) / dt;
    }

    let output: f32 = kp * error + ki * *integral + kd * derivative;
    *prev_error = error;
    Ok(output as i32)
}

// clamps a value between min and max
fn clamp(v: i32, min: i32, max: i32) -> i32 {
    if v < min { min } else if v > max { max } else { v }
}

// function to get current time in milliseconds
fn current_time_millis() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as i64
}


//  "task main()" is executed at programm start
fn main() -> Ev3Result<()> {

    // Constants
    let light_left = LightSensor::get(SensorPort::In1)?;
    let light_right = LightSensor::get(SensorPort::In2)?;
    let motor = LargeMotor::get(MotorPort::OutA)?;

    // PID constants
    // for some reason when using rust the sensors goes from 0 to 1000
    //  while in Python they go from 0 to 100
    //  so we need to adjust the the terms accordingly
    let kp: f32 = 0.1;
    let ki: f32 = 0.1;
    let kd: f32 = 0.1;

    // PID variables
    let mut prev_error: f32 = 0.0;
    let mut integral: f32 = 0.0;

    // variable to store pidController output
    motor.run_direct()?;
    light_left.set_mode_reflect()?;
    light_right.set_mode_reflect()?;
    let mut motor_power: i32;
    let mut dt: f32;
    let mut current_time: i64;
    let mut last_time: i64;

    last_time = current_time_millis();

    //  loops forever
    loop {
        // Time difference
        current_time = current_time_millis();
        dt = (current_time - last_time) as f32 / 1000.0; // in seconds
        last_time = current_time;

        // Compute PID output
        motor_power = pid_controller(light_left.get_light_intensity()?, light_right.get_light_intensity()?, 
                                    dt, &mut integral, &mut prev_error, &kp, &ki, &kd)?;

        // set motor speed
        motor.set_duty_cycle_sp(clamp(motor_power, -100, 100))?;

        // wait for 100 ms
        thread::sleep(Duration::from_millis(100));
    }
}