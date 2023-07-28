cmake_minimum_required(VERSION 3.10)

project(molpack)


set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(${PROJECT_NAME} src/sarw.cpp)
target_include_directories(${PROJECT_NAME} PUBLIC include)

add_executable(${PROJECT_NAME}_bin src/molpack.cpp)
target_link_libraries(${PROJECT_NAME}_bin PUBLIC ${PROJECT_NAME})

enable_testing()

include(FetchContent)
FetchContent_Declare(
  DocTest
  GIT_REPOSITORY https://github.com/doctest/doctest.git
  GIT_TAG        v2.4.11
)
FetchContent_MakeAvailable(DocTest)

function(molpack_test target_name)
    add_executable(${target_name}_test test/${target_name}_test.cpp)
    add_test(NAME ${target_name}_test COMMAND ${target_name}_test)
    target_link_libraries(${target_name}_test PUBLIC doctest::doctest ${PROJECT_NAME})
endfunction()

molpack_test(vec3)
molpack_test(region)
molpack_test(cell_list)
molpack_test(sarw)