cmake_minimum_required(VERSION 3.10)

project(MolPack)


set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(${PROJECT_NAME} src/molpack.cpp src/sarw.cpp)
include_directories(PRIVATE include)

enable_testing()

include(FetchContent)
FetchContent_Declare(
  DocTest
  GIT_REPOSITORY https://github.com/doctest/doctest.git
  GIT_TAG        v2.4.11
)
FetchContent_MakeAvailable(DocTest)


add_executable(vector3d_test test/vector3d_test.cpp)
add_test(NAME vector3d_test COMMAND vector3d_test)
target_link_libraries(vector3d_test PUBLIC doctest::doctest)

add_executable(region_test test/region_test.cpp)
add_test(NAME region_test COMMAND region_test)
target_link_libraries(region_test PUBLIC doctest::doctest)

add_executable(cell_list_test test/cell_list_test.cpp)
add_test(NAME cell_list_test COMMAND cell_list_test)
target_link_libraries(cell_list_test PUBLIC doctest::doctest)

add_executable(sarw_test test/sarw_test.cpp src/sarw.cpp)
add_test(NAME sarw_test COMMAND sarw_test)
target_link_libraries(sarw_test PUBLIC doctest::doctest)