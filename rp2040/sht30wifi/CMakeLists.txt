cmake_minimum_required(VERSION 3.12)

include($ENV{PICO_SDK_PATH}/pico_sdk_init.cmake)
set(CMAKE_C_STANDARD 11)
set(PICO_BOARD "pico_w")

project(sht30wifi)
pico_sdk_init()

add_executable(sht30wifi main.c)
target_include_directories(sht30wifi PRIVATE ${CMAKE_CURRENT_LIST_DIR})
target_link_libraries(sht30wifi pico_stdlib hardware_i2c pico_cyw43_arch_lwip_threadsafe_background)

pico_enable_stdio_usb(sht30wifi 1)
pico_add_extra_outputs(sht30wifi)
