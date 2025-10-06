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


from time import sleep
from ev3dev2.motor import OUTPUT_A, LargeMotor
from ev3dev2.sensor import INPUT_1, INPUT_2
from ev3dev2.sensor.lego import LightSensor


# Constants
LIGHT_LEFT = LightSensor(INPUT_1)
LIGHT_RIGHT = LightSensor(INPUT_2)
MOTOR = LargeMotor(OUTPUT_A)
P_TERM = 1.0


def p_controller(sensor_left: LightSensor, sensor_right: LightSensor, p_term: float) -> float:
    difference = sensor_left.reflected_light_intensity - sensor_right.reflected_light_intensity
    return difference * p_term


# is executed when the program is started
if __name__ == "__main__":

    # variable to store p_controller output
    motor_power = 0.0

    # loops forever
    while True:
        motor_power = p_controller(LIGHT_LEFT, LIGHT_RIGHT, P_TERM)
        MOTOR.on(motor_power)
        sleep(0.1)