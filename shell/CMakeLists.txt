cmake_minimum_required(VERSION 3.19)
project(shell C)

set(CMAKE_C_STANDARD 11)

add_executable(shell key_reader.c key_reader.c key_reader.h)
