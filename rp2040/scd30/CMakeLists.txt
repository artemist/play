cmake_minimum_required(VERSION 3.12)

include($ENV{PICO_SDK_PATH}/pico_sdk_init.cmake)
set(CMAKE_C_STANDARD 11)

project(scd30)
pico_sdk_init()

add_executable(scd30
        scd30.c
        )
target_link_libraries(scd30 pico_stdlib hardware_i2c)
pico_enable_stdio_usb(scd30 1)
pico_add_extra_outputs(scd30)
