cmake_minimum_required(VERSION 3.21)
project(slingshot
    LANGUAGES C CXX
    VERSION 1.0.0
    DESCRIPTION "SystemVerilog language server"
    HOMEPAGE_URL "https://github.com/mattyoung101/slingshot"
)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)
set(CMAKE_C_STANDARD 11) # C11
set(CMAKE_CXX_STANDARD 20) # C++20
cmake_policy(SET CMP0144 NEW)

include(GNUInstallDirs)
include(CPack)

add_executable(slingshot
    src/main.cpp
    src/handlers.cpp
)
target_include_directories(slingshot PRIVATE include)

find_package(spdlog REQUIRED)
find_package(fmt REQUIRED)
find_package(slang REQUIRED)
find_package(LLVM REQUIRED)
find_package(lsp REQUIRED)
find_package(SQLiteCpp)
find_package(SQLite3)
target_link_libraries(slingshot PRIVATE spdlog::spdlog fmt::fmt slang::slang lsp::lsp SQLiteCpp SQLite::SQLite3)

# note on diagnostic colour: https://stackoverflow.com/a/73349744/5007892
target_compile_options(slingshot PRIVATE "-Wall" "-Wextra" "-Wno-unused-parameter" "-ggdb"
    "-fdiagnostics-color=always")
target_compile_definitions(slingshot PRIVATE "-DSPDLOG_ACTIVE_LEVEL=SPDLOG_LEVEL_TRACE")

#### LINKER
message(STATUS "Checking if LLD is available")
execute_process(COMMAND "lld" ERROR_VARIABLE LLD_RESULT)

if (LLD_RESULT MATCHES "lld is a generic driver")
    message(STATUS "LLD is available, it will be used")
    target_link_options(slingshot PRIVATE "-fuse-ld=lld")
else()
    message(STATUS "LLD is NOT available, so we'll use the system default linker")
endif()

## SANITIZERS
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    message(STATUS "Debug build, adding sanitizers")

    target_compile_options(slingshot PRIVATE "-fsanitize=address" "-fsanitize=undefined")

    target_link_options(slingshot PRIVATE "-fsanitize=address" "-fsanitize=undefined")
endif()

## INSTALL
install(TARGETS slingshot)
