cmake_minimum_required(VERSION 3.10)

project(molpack)


set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)

option(MOLPACK_BUILD_TESTS "Build tests" OFF)

add_library(${PROJECT_NAME} src/sarw.cpp)
target_include_directories(${PROJECT_NAME} PUBLIC include)

add_executable(${PROJECT_NAME}_bin src/molpack.cpp)
set_target_properties(${PROJECT_NAME}_bin PROPERTIES OUTPUT_NAME ${PROJECT_NAME})
target_link_libraries(${PROJECT_NAME}_bin PUBLIC ${PROJECT_NAME})

if(MOLPACK_BUILD_TESTS)
    enable_testing()
    add_subdirectory(test)
endif()