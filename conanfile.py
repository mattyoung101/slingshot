from conan import ConanFile

class SlingshotConan(ConanFile):
    generators = ["CMakeDeps", "CMakeToolchain"]
    requires = [
        "spdlog/1.16.0",
        "slang/9.1",
        "yaml-cpp/0.8.0",
        "lsp-framework/1.2.0",
        "unordered_dense/4.5.0",
        "nlohmann_json/3.12.0",
    ]
    settings = "os", "arch", "compiler", "build_type"

    def requirements(self):
        self.requires("fmt/12.1.0", override=True)
        # # force this to try and fix https://github.com/Z3Prover/z3/issues/7398
        # self.requires("z3/4.14.1", override=True)
