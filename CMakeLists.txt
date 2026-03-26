cmake_minimum_required(VERSION 3.21)
project(slingshot
    LANGUAGES C CXX
    VERSION 1.2.4
    DESCRIPTION "SystemVerilog language server powered by Slang"
    HOMEPAGE_URL "https://github.com/mlyoung101/slingshot"
)
set(CPACK_PACKAGE_CONTACT "M. L. Young <foss@mlyoung.cool>")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

if (CMAKE_BUILD_TYPE STREQUAL "Release")
    message(STATUS "Enable LTO")
    set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
endif()

set(CMAKE_C_STANDARD 11) # C11
set(CMAKE_CXX_STANDARD 23) # C++23
cmake_policy(SET CMP0144 NEW)

include(CPack)
include(cmake/CPM.cmake)

option(SLINGSHOT_ENABLE_REMOTE_DEBUGGER "Enables the remote debugger functionality in Slingshot. Does not work
    on Windows." TRUE)

## LINKER
message(STATUS "Checking if mold is available")
execute_process(COMMAND "mold" "--version" OUTPUT_VARIABLE MOLD_RESULT)

## DEPS
CPMAddPackage("gh:gabime/spdlog@1.16.0")
CPMAddPackage("gh:fmtlib/fmt#12.1.0")
CPMAddPackage(URI "gh:MikePopoloski/slang@10.0" OPTIONS "SLANG_USE_MIMALLOC OFF" "SLANG_USE_THREADS OFF")
CPMAddPackage("gh:martinus/unordered_dense@4.8.1")
CPMAddPackage("gh:nlohmann/json@3.12.0")
CPMAddPackage("gh:leon-bckl/lsp-framework#1.3.0")
CPMAddPackage("gh:bshoshany/thread-pool@5.0.0")
CPMAddPackage("gh:marzer/tomlplusplus@3.4.0")
if (SLINGSHOT_ENABLE_REMOTE_DEBUGGER)
    CPMAddPackage(URI "gh:fpagliughi/sockpp#v1.x" OPTIONS "SOCKPP_BUILD_STATIC ON")
endif()
CPMAddPackage(URI "gh:Precision-Contribution-Group/graaf#b82914d7f83d9710be80965425c49899b99085d5"
    OPTIONS "SKIP_TESTS ON" "SKIP_EXAMPLES ON" "SKIP_BENCHMARKS ON")
# CPMAddPackage("gh:yohhoy/yamc#4e015a7e8eb0d61c34e6928676c8c78881a72d73")
CPMAddPackage("gh:catchorg/Catch2@3.11.0")

add_library(BS_thread_pool INTERFACE)
target_include_directories(BS_thread_pool INTERFACE ${BS_thread_pool_SOURCE_DIR}/include)

list(APPEND CMAKE_MODULE_PATH ${Catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)

# Adds a Slingshot-like target with the specified suffix and compile args
# ref: https://hsf-training.github.io/hsf-training-cmake-webpage/11-functions/index.html
function(add_slingshot_target TARGET)
    cmake_parse_arguments(PARSE_ARGV 1 SLING "" "" "EXTRA_ARGS")
    message(STATUS "Target name: ${TARGET}")
    message(STATUS "Extra args: ${SLING_EXTRA_ARGS}")

    add_executable(${TARGET}
        src/main.cpp
        src/handlers.cpp
        src/indexing.cpp
        src/compiler.cpp
        src/remote_debug.cpp
        src/completion.cpp
        src/completion_generator.cpp
        src/lang_lifter.cpp
        src/import_locator.cpp
        src/document_graph.cpp
    )
    target_include_directories(${TARGET} PRIVATE include)

    message(STATUS "Remote debugger enabled: ${SLINGSHOT_ENABLE_REMOTE_DEBUGGER}")

    target_link_libraries(${TARGET} PRIVATE spdlog::spdlog fmt::fmt slang::slang lsp
        unordered_dense::unordered_dense nlohmann_json::nlohmann_json BS_thread_pool tomlplusplus::tomlplusplus
        Graaf::Graaf)

    if (SLINGSHOT_ENABLE_REMOTE_DEBUGGER)
        target_link_libraries(${TARGET} PRIVATE sockpp-static)
    endif()

    # note on diagnostic colour: https://stackoverflow.com/a/73349744/5007892
    target_compile_options(${TARGET} PRIVATE "-Wall" "-Wextra" "-Wno-unused-parameter" "-ggdb"
        "-fdiagnostics-color=always")
    target_compile_definitions(${TARGET} PRIVATE
        "-DSPDLOG_ACTIVE_LEVEL=SPDLOG_LEVEL_TRACE"
        "-DSLINGSHOT_VERSION=\"${CMAKE_PROJECT_VERSION}\""
    )

    if (SLINGSHOT_ENABLE_REMOTE_DEBUGGER)
        target_compile_definitions(${TARGET} PRIVATE "-DSLINGSHOT_ENABLE_REMOTE_DEBUGGER=1")
    else()
        target_compile_definitions(${TARGET} PRIVATE "-DSLINGSHOT_ENABLE_REMOTE_DEBUGGER=0")
    endif()

    #### LINKER
    if (MOLD_RESULT MATCHES "mold 2")
        message(STATUS "mold is available, it will be used")
        target_link_options(${TARGET} PRIVATE "-fuse-ld=mold")
    else()
        message(STATUS "mold is NOT available, so we'll use the system default linker")
    endif()

    ## EXTRA ARGS
    target_compile_options(${TARGET} PRIVATE ${SLING_EXTRA_ARGS})
    target_link_options(${TARGET} PRIVATE ${SLING_EXTRA_ARGS})

    ## INSTALL
    install(TARGETS ${TARGET})
endfunction()


# Adds a Slingshot test
function(add_slingshot_test TARGET SOURCE)
    message(STATUS "Add test: ${TARGET} Source file: ${SOURCE}")

    # TODO compile a generic test library that includes all of this
    add_executable(${TARGET}
        src/handlers.cpp
        src/indexing.cpp
        src/compiler.cpp
        src/remote_debug.cpp
        src/completion.cpp
        src/completion_generator.cpp
        src/lang_lifter.cpp
        src/import_locator.cpp
        src/document_graph.cpp
        test/globals.cpp
        ${SOURCE}
    )
    target_include_directories(${TARGET} PRIVATE include)

    target_link_libraries(${TARGET} PRIVATE spdlog::spdlog fmt::fmt slang::slang lsp
        unordered_dense::unordered_dense nlohmann_json::nlohmann_json BS_thread_pool tomlplusplus::tomlplusplus
        Graaf::Graaf Catch2::Catch2WithMain)

    if (SLINGSHOT_ENABLE_REMOTE_DEBUGGER)
        target_link_libraries(${TARGET} PRIVATE sockpp-static)
    endif()

    # note on diagnostic colour: https://stackoverflow.com/a/73349744/5007892
    target_compile_options(${TARGET} PRIVATE "-Wall" "-Wextra" "-Wno-unused-parameter" "-ggdb"
        "-fdiagnostics-color=always")
    target_compile_definitions(${TARGET} PRIVATE
        "-DSPDLOG_ACTIVE_LEVEL=SPDLOG_LEVEL_TRACE"
        "-DSLINGSHOT_VERSION=\"${CMAKE_PROJECT_VERSION}\""
    )

    if (SLINGSHOT_ENABLE_REMOTE_DEBUGGER)
        target_compile_definitions(${TARGET} PRIVATE "-DSLINGSHOT_ENABLE_REMOTE_DEBUGGER=1")
    else()
        target_compile_definitions(${TARGET} PRIVATE "-DSLINGSHOT_ENABLE_REMOTE_DEBUGGER=0")
    endif()

    #### LINKER
    if (MOLD_RESULT MATCHES "mold 2")
        message(STATUS "mold is available, it will be used")
        target_link_options(${TARGET} PRIVATE "-fuse-ld=mold")
    else()
        message(STATUS "mold is NOT available, so we'll use the system default linker")
    endif()

    catch_discover_tests(${TARGET})
endfunction()

########################################################################################################
########################################################################################################
########################################################################################################

# Base target, in all builds, with no instrumentation
add_slingshot_target(slingshot)

# Sanitized targets
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_slingshot_target(slingshot_asan EXTRA_ARGS "-O0" "-fsanitize=address" "-fsanitize=undefined")
    add_slingshot_target(slingshot_tsan EXTRA_ARGS "-O0" "-fsanitize=thread")
    add_slingshot_target(slingshot_msan EXTRA_ARGS "-O0" "-fsanitize=memory")
endif()

add_slingshot_test(sling_test_indexer test/indexer.cpp)
