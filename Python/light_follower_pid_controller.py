#!/usr/bin/env python3

"""
Light Follower Robot Python example
Author: David Laurent Reinhardt
Course: "System Design Projekt", Winter Semester 2025/2026

Description:
This program implements a simple light-following robot using a proportional controller.
The robot uses two light sensors to detect the intensity of light on its left and right sides.
Based on the difference in light intensity, it adjusts its motor speed to turn towards the light source.
"""


from time import sleep, time
from ev3dev2.motor import OUTPUT_A, LargeMotor
from ev3dev2.sensor import INPUT_1, INPUT_2
from ev3dev2.sensor.lego import LightSensor

# Constants
LIGHT_LEFT = LightSensor(INPUT_1)
LIGHT_RIGHT = LightSensor(INPUT_2)
MOTOR = LargeMotor(OUTPUT_A)

# PID constants
KP = 1.0   # Proportional gain
KI = 1.0   # Integral gain
KD = 1.0  # Derivative gain


# --- PID Controller Function ---
def pidController(sensor_left: int, sensor_right: int, prev_error: float, integral: float, dt: float):
    error = sensor_right - sensor_left
    integral += error * dt
    derivative = (error - prev_error) / dt if dt > 0 else 0
    output = KP * error + KI * integral + KD * derivative
    return output, error, integral


# is executed at programm start
if __name__ == "__main__":

    prev_error = 0.0
    integral = 0.0
    last_time = time()

    while True:
        # Time difference
        current_time = time()
        dt = current_time - last_time
        last_time = current_time

        # Read sensor values
        left_value = LIGHT_LEFT.reflected_light_intensity
        right_value = LIGHT_RIGHT.reflected_light_intensity

        # Compute motor power using PID
        motor_power, prev_error, integral = pidController(left_value, right_value, prev_error, integral, dt)

        # Apply motor output
        MOTOR.on(motor_power)

        # Run loop every 100 ms
        sleep(0.1)