#include "slang/util/VersionInfo.h"
#include "spdlog/spdlog.h"

int main() {
    using namespace slang;

    SPDLOG_INFO("Slang version: {}.{}", VersionInfo::getMajor(), VersionInfo::getMinor());

    return 0;
}
