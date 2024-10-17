cmake_minimum_required(VERSION 3.28)
project(UPRE_Task)

set(CMAKE_CXX_STANDARD 17)

include_directories(Inc)

file(GLOB SOURCE "Src/*.*")

add_executable(UPRE_Task ${SOURCE})
