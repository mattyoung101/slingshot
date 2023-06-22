cmake_minimum_required(VERSION 3.26)
project(slingshot)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# YOU WILL NOT LINK TIME OPTIMISE MY PROJECT!!!!!!
# This mega breaks Rust for some reason, so we cannot use it.
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION FALSE)
set(FETCHCONTENT_QUIET FALSE)
include(FetchContent)

# Optimisation
if ("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    message(STATUS "Release build, enabling performance")
    add_compile_options(-O3 -march=native -mtune=native)
    # add_link_options()
else()
    message(STATUS "Debug build, enabling sanitizers")
    add_compile_options(-Og)
    
    # the below messes with rust but is necessary for debugging the C++ directly
    #add_compile_options(-Og -fsanitize=address -fsanitize=undefined -fno-omit-frame-pointer)
    #add_link_options(-fsanitize=address -fsanitize=undefined)
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
# TODO: configure mimalloc to work correctly (may be faster -> although possible penalty for maintaining two
# heaps, one for gcc and one for slang, we should move to mimalloc entirely)
set(SLANG_USE_MIMALLOC OFF)
FetchContent_MakeAvailable(slang)

# we'll have the following targets:
# - slingshot tinker executable -> for quick local testing of the API
# - slingshot library -> for linking with Rust
# - slingshot unit tests -> for running unit tests

# add the slingshot static library
add_library(slingshot STATIC src/slingshot.cpp)
target_include_directories(slingshot PUBLIC include)
target_link_libraries(slingshot PUBLIC slang::slang)
target_compile_options(slingshot PRIVATE "-Wall" "-Wextra" "-g3")
# Only compile Slingshot with C++20, allow dependencies to choose their own C++ version
set_target_properties(slingshot PROPERTIES 
    CXX_STANDARD 20 
    CXX_EXTENSIONS ON 
    INTERPROCEDURAL_OPTIMIZATION OFF
)

# add the slingshot executable
add_executable(slingshot_tinker src/main.cpp)
target_include_directories(slingshot_tinker PRIVATE include)
target_link_libraries(slingshot_tinker PRIVATE slang::slang slingshot)
target_compile_options(slingshot_tinker PRIVATE "-Wall" "-Wextra" "-g3")
# Only compile Slingshot with C++20, allow dependencies to choose their own C++ version
set_target_properties(slingshot_tinker PROPERTIES 
    CXX_STANDARD 20 
    CXX_EXTENSIONS ON 
    INTERPROCEDURAL_OPTIMIZATION OFF
)

# TODO unit tests

# this is just to make the rust build script behave, the C++ project isn't designed for installation
install(TARGETS slingshot)
# source: https://stackoverflow.com/a/40685598/5007892
# TODO is this required?
install(
    DIRECTORY include
    DESTINATION include
    FILES_MATCHING PATTERN "*.h*"
)

