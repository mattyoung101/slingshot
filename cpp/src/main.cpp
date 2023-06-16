#include <iostream>
#include "slingshot/token_extractor.hpp"
#include "slingshot/slingshot.hpp"

using namespace slingshot;

int main(int argc, char *argv[]) {
    std::cout << "Slingshot CPP version: " << SLINGSHOT_CPP_VERSION << std::endl;
    std::cout << "Running token extractor" << std::endl;

    CompletionTokenExtractor extractor;
    extractor.extractTokens("module m; endmodule");

    return 0;
}
