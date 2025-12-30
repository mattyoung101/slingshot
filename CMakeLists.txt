cmake_minimum_required(VERSION 3.21)
project(slingshot
    LANGUAGES C CXX
    VERSION 1.0.0
    DESCRIPTION "SystemVerilog language server"
    HOMEPAGE_URL "https://github.com/mattyoung101/slingshot"
)
set(CPACK_PACKAGE_CONTACT "Matt Young <matt@mlyoung.cool>")
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

add_executable(slingshot
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
target_include_directories(slingshot PRIVATE include)

# find deps
CPMAddPackage("gh:gabime/spdlog@1.16.0")
CPMAddPackage("gh:fmtlib/fmt#12.1.0")
CPMAddPackage(URI "gh:MikePopoloski/slang@9.1" OPTIONS "SLANG_USE_MIMALLOC OFF" "SLANG_USE_THREADS OFF")
CPMAddPackage("gh:martinus/unordered_dense@4.8.1")
CPMAddPackage("gh:nlohmann/json@3.12.0")
CPMAddPackage("gh:leon-bckl/lsp-framework#1.3.0")
CPMAddPackage("gh:bshoshany/thread-pool@5.0.0")
CPMAddPackage("gh:marzer/tomlplusplus@3.4.0")
CPMAddPackage(URI "gh:fpagliughi/sockpp#v1.x" OPTIONS "SOCKPP_BUILD_STATIC ON")
CPMAddPackage(URI "gh:bobluppes/graaf#e0ec371ae8266fb72d986c0de5c47286f34a0589" OPTIONS "SKIP_TESTS ON"
    "SKIP_EXAMPLES ON" "SKIP_BENCHMARKS ON")

add_library(BS_thread_pool INTERFACE)
target_include_directories(BS_thread_pool INTERFACE ${BS_thread_pool_SOURCE_DIR}/include)

target_link_libraries(slingshot PRIVATE spdlog::spdlog fmt::fmt slang::slang lsp
    unordered_dense::unordered_dense nlohmann_json::nlohmann_json BS_thread_pool tomlplusplus::tomlplusplus
    sockpp-static Graaf::Graaf)

# note on diagnostic colour: https://stackoverflow.com/a/73349744/5007892
target_compile_options(slingshot PRIVATE "-Wall" "-Wextra" "-Wno-unused-parameter" "-ggdb"
    "-fdiagnostics-color=always")
target_compile_definitions(slingshot PRIVATE "-DSPDLOG_ACTIVE_LEVEL=SPDLOG_LEVEL_TRACE")

#### LINKER
message(STATUS "Checking if mold is available")
execute_process(COMMAND "mold" "--version" OUTPUT_VARIABLE MOLD_RESULT)

if (MOLD_RESULT MATCHES "mold 2")
    message(STATUS "mold is available, it will be used")
    target_link_options(slingshot PRIVATE "-fuse-ld=mold")
else()
    message(STATUS "mold is NOT available, so we'll use the system default linker")
endif()

## SANITIZERS
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    message(STATUS "Debug build, adding sanitizers")
    target_compile_options(slingshot PRIVATE "-fsanitize=address" "-fsanitize=undefined")
    target_link_options(slingshot PRIVATE "-fsanitize=address" "-fsanitize=undefined")
endif()

## INSTALL
install(TARGETS slingshot)
