cmake_minimum_required(VERSION 3.26)
project(slingshot)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# YOU WILL NOT LINK TIME OPTIMISE MY PROJECT!!!!!!
# This mega breaks Rust for some reason, so we cannot use it.
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION FALSE)

set(FETCHCONTENT_QUIET FALSE)
include(FetchContent)

# split project into multiple cmake scripts: https://stackoverflow.com/a/69545109/5007892
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(MergeStaticLibraries)

# Optimisation
if ("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    message(STATUS "Release build, enabling performance")
    add_compile_options(-O3 -march=native -mtune=native)
    # add_link_options()
else()
    message(STATUS "Debug build, enabling sanitizers")
    add_compile_options(-Og)
    
    # the below messes with rust but is necessary for debugging the C++ directly
    # TODO add a USE_SANITIZERS option to enable this shit

    #add_compile_options(-Og -fsanitize=address -fsanitize=undefined -fno-omit-frame-pointer)
    #add_link_options(-fsanitize=address -fsanitize=undefined)
endif()

# NOTE: We no longer pull Catch2 or Slang dependencies because rust-cmake is incapable of building them
# correctly, and even if it does, since it's recompiling both each time it takes fuckin forever. 
# You will have to build these dependencies yourself, see README.
find_package(slang REQUIRED)

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

# build a fat static library with all dependencies linked in, for then linking with Rust
bundle_static_library(slingshot slingshotfat)

# TODO unit tests -> maybe we want them from the rust side?

# install dependencies
# this is just to make the rust build script behave, the C++ project isn't designed for installation
install(TARGETS slingshot)
# source: https://stackoverflow.com/a/40685598/5007892
# FIXME is this required?
install(
    DIRECTORY include
    DESTINATION include
    FILES_MATCHING PATTERN "*.h*"
)
# install the fat library, so that Rust can link against it
install(FILES ${CMAKE_BINARY_DIR}/libslingshotfat.a DESTINATION lib)
