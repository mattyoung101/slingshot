#pragma once
#include "slingshot-cpp/include/slingshot/slingshot.h"

// This is not documented well, but "wrapper.hpp" MUST be in the root directory of the project and
// include the actual header file you want. Otherwise, bindgen literally will not work. And, no, you can't
// just ask bindgen to include files from another directory, as far as I can tell for whatever reason, it MUST
// be the root.
