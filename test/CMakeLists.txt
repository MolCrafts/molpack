cmake_minimum_required(VERSION 3.10)

project(molpack_test)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_WARN_DEPRECATED OFF CACHE BOOL "" FORCE)

include(FetchContent)
FetchContent_Declare(
  DocTest
  GIT_REPOSITORY https://github.com/doctest/doctest.git
  GIT_TAG        v2.4.11
)
FetchContent_MakeAvailable(DocTest)

add_executable(${PROJECT_NAME}
    main.cpp
    cell_list.cpp
    region.cpp
    sarw.cpp
    vec3.cpp
    mat3.cpp
    geom.cpp
)
target_link_libraries(${PROJECT_NAME} PRIVATE doctest::doctest molpack)
include(${doctest_SOURCE_DIR}/scripts/cmake/doctest.cmake)
doctest_discover_tests(${PROJECT_NAME})