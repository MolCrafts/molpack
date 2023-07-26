cmake_minimum_required(VERSION 3.10)

project(MolPack)


set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(${PROJECT_NAME} src/molpack.cpp src/sarw.cpp)
target_include_directories(${PROJECT_NAME} PRIVATE include)

enable_testing()

# include(ExternalProject)
# find_package(Git REQUIRED)
# ExternalProject_Add(
#     doctest
#     PREFIX ${CMAKE_BINARY_DIR}/doctest
#     GIT_REPOSITORY https://github.com/doctest/doctest.git
#     TIMEOUT 10
#     UPDATE_COMMAND ${GIT_EXECUTABLE} pull
#     CONFIGURE_COMMAND ""
#     BUILD_COMMAND ""
#     INSTALL_COMMAND ""
#     LOG_DOWNLOAD ON
# )

# # Expose required variable (DOCTEST_INCLUDE_DIR) to parent scope
# ExternalProject_Get_Property(doctest source_dir)
# set(DOCTEST_INCLUDE_DIR ${source_dir}/doctest CACHE INTERNAL "Path to include folder for doctest")

include(FetchContent)
FetchContent_Declare(
  doctest
  GIT_REPOSITORY https://github.com/doctest/doctest.git
  GIT_TAG        v2.4.11
)
FetchContent_MakeAvailable(doctest)


add_executable(vector3d_test test/vector3d_test.cpp)
add_test(NAME vector3d_test COMMAND vector3d_test)
target_include_directories(vector3d_test PRIVATE ${DOCTEST_INCLUDE_DIR} include)

add_executable(region_test test/region_test.cpp)
add_test(NAME region_test COMMAND region_test)
target_include_directories(region_test PRIVATE ${DOCTEST_INCLUDE_DIR} include)