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
P_TERM: float = 1.0


# p(roportional)-controller function
def p_controller(sensor_left: int, sensor_right: int, p_term: float) -> float:
    difference: float = sensor_left - sensor_right
    return p_term * difference


# is executed when the program is started
if __name__ == "__main__":

    # variable to store p_controller output
    motor_power: float

    # loops forever
    while True:
        
        motor_power = p_controller(LIGHT_LEFT.reflected_light_intensity, LIGHT_RIGHT.reflected_light_intensity, P_TERM)
        MOTOR.on(motor_power)

        # wait for 100 milliseconds
        sleep(0.1)