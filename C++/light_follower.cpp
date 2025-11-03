/* 
COMPLETLY UNTESTED CODE
Light Follower Robot C++ example
Author: David Laurent Reinhardt
Course: "System Design Projekt", Winter Semester 2025/2026

Description:
This program implements a simple light-following robot using a proportional controller.
The robot uses two light sensors to detect the intensity of light on its left and right sides.
Based on the difference in light intensity, it adjusts its motor speed to turn towards the light source.
*/

#include "ev3dev.h"
#include <unistd.h>
#include <iostream>

// Constants
const float P_TERM = 1.0f;


//  p(roportional)-controller
float p_controller(int input_1, int input_2, float p_term) {
    float difference = input_1 - input_2;
    return p_term * difference;
}


//  "task main()" is executed at program start
int main() {
    // Initialize sensors and motors
    ev3dev::light_sensor left_sensor(ev3dev::INPUT_1);
    ev3dev::light_sensor right_sensor(ev3dev::INPUT_2);
    ev3dev::large_motor motor(ev3dev::OUTPUT_A);
    motor.run_direct();

    // Main control loop
    while (true) {

        // Compute motor speed using proportional controller
        float speed = p_controller(left_sensor.reflected_light_intensity(), right_sensor.reflected_light_intensity(), P_TERM);
        motor.set_duty_cycle_sp(static_cast<int>(speed));

        // wait for 100 milliseconds
        usleep(100000);
    }

    return 0;
}