cmake_minimum_required(VERSION 3.26)
project(slingshot)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
set(FETCHCONTENT_QUIET FALSE)
include(FetchContent)

# Optimisation
if ("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    message(STATUS "Release build, enabling performance")
    add_compile_options(-O3 -march=native -mtune=native -flto)
    add_link_options(-flto)
else()
    message(STATUS "Debug build, enabling sanitizers")
    add_compile_options(-Og -fsanitize=address -fsanitize=undefined -fno-omit-frame-pointer)
    add_link_options(-fsanitize=address -fsanitize=undefined)
endif()

# Pull Catch2 dependency for unit testing
FetchContent_Declare(
    Catch2
    GIT_REPOSITORY https://github.com/catchorg/Catch2.git
    GIT_TAG        v3.3.2 
    GIT_PROGRESS TRUE
)
FetchContent_MakeAvailable(Catch2)

# Pull slang dependency (core dependency for SV parsing)
# Note that we use the latest commit on master (updated manually by me) because the current 3.0 release has
# a bug that prevents compilation under Clang.
FetchContent_Declare(
    slang
    GIT_REPOSITORY https://github.com/MikePopoloski/slang.git
    GIT_TAG 5e1f963e246f5fc1e2bc2fee4d32e9ad66b4c1d9
    GIT_PROGRESS TRUE
)
# Currently fails because mimalloc-static cannot be found on my system
# TODO: configure mimalloc to work correctly
set(SLANG_USE_MIMALLOC OFF)
FetchContent_MakeAvailable(slang)

# See if we can use the Mold linker
find_program(MOLD mold)

# we'll have two targets:
# - slingshot test executable
# - slingshot library

# add the slingshot executable
include_directories(cpp/include)
add_executable(slingshot cpp/src/main.cpp cpp/src/slingshot.cpp)
target_link_libraries(slingshot PRIVATE slang::slang)
target_compile_options(slingshot PRIVATE "-Wall" "-Wextra" "-g3")
# Only compile Slingshot with C++20, allow dependencies to choose their own C++ version
set_target_properties(slingshot PROPERTIES 
    CXX_STANDARD 20 
    CXX_EXTENSIONS ON 
    INTERPROCEDURAL_OPTIMIZATION ON
)

# Use mold for faster linking if available
if (MOLD)
    message(STATUS "Using mold linker")
    target_link_options(slingshot PRIVATE "-fuse-ld=mold")
else()
    message(WARNING "NOT using mold linker")
endif()
