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
#include <algorithm>

// Constants
#define P_TERM 1.0f


//  p(roportional)-controller
float p_controller(int input_1, int input_2, float p_term) {
    float difference = input_1 - input_2;
    return p_term * difference;
}


//  "int main()" is executed at program start
int main() {
    using namespace ev3dev;

    // Initialize sensors and motors
    color_sensor left_sensor(INPUT_1);
    color_sensor right_sensor(INPUT_2);
    left_sensor.set_mode("COL-REFLECT");
    right_sensor.set_mode("COL-REFLECT");
    motor motor(OUTPUT_A);
    motor.run_direct();

    // variable to store p_controller output
    float motor_power;

    // Main control loop
    while (true) {

        // Compute motor speed using proportional controller
        motor_power = p_controller(left_sensor.reflected_light_intensity(), right_sensor.reflected_light_intensity(), P_TERM);
        
        // Set motor speed with clamping to valid range
        int motor_power_clamped = static_cast<int>(std::clamp(motor_power, -100.0f, 100.0f));
        motor.set_duty_cycle_sp(motor_power_clamped);

        // wait for 100 milliseconds
        usleep(100000);
    }

    return 0;
}