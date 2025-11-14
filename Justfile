default:
    @just -l

conan:
    conan install . --output-folder=build --build=missing --profile=default \
        -c tools.system.package_manager:mode=install -c \
        tools.system.package_manager:sudo=True

conan_debug:
    conan install . --output-folder=build_debug --build=missing --profile=debug \
        -c tools.system.package_manager:mode=install -c \
        tools.system.package_manager:sudo=True

cmake: conan
    cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
        -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_C_COMPILER_LAUNCHER=ccache -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake"

cmake_debug: conan_debug
    cmake -B build_debug -G Ninja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
        -DCMAKE_CXX_COMPILER_LAUNCHER=ccache -DCMAKE_C_COMPILER_LAUNCHER=ccache -DCMAKE_TOOLCHAIN_FILE="conan_toolchain.cmake"

build: cmake
    cd build; ninja

build_debug: cmake_debug
    cd build_debug; ninja
